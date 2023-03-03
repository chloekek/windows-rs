// mod syntax;
use metadata::reader;

enum ArgKind {
    None,
    Input,
    Output,
    Include,
    Exclude,
}

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

type ToolResult<T> = std::result::Result<T, String>;

fn run() -> ToolResult<()> {
    let time = std::time::Instant::now();
    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!(
            r#"Usage: riddle.exe [options...]

Options:
  -input   <path>       Path to files and directories containing .winmd and .idl files
  -output  <path>       Path to .winmd or idl file to generate
  -include <namespace>  Namespaces to include in output (defaults to everything) 
  -exclude <namespace>  Namespaces to exclude in output (defaults to nothing)
  -fmt                  Format .idl files only
  -verbose              Show detailed information
"#
        );
        return Ok(());
    }

    let mut kind = ArgKind::None;
    let mut output = String::new();
    let mut input = Vec::<String>::new();
    let mut include = Vec::<String>::new();
    let mut exclude = Vec::<String>::new();
    let mut fmt = false;
    let mut verbose = false;

    for arg in args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "-input" => kind = ArgKind::Input,
                "-output" => kind = ArgKind::Output,
                "-include" => kind = ArgKind::Include,
                "-exclude" => kind = ArgKind::Exclude,
                "-fmt" => fmt = true,
                "-verbose" => verbose = true,
                _ => return Err(format!("invalid option: `{}`", arg)),
            },
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg;
                } else {
                    return Err("too many outputs".to_string());
                }
            }
            ArgKind::Input => input.push(arg),
            ArgKind::Include => include.push(arg),
            ArgKind::Exclude => exclude.push(arg),
        }
    }

    if fmt {
        if !output.is_empty() || !include.is_empty() || !exclude.is_empty() {
            return Err("-fmt cannot be combined with -output, -include, or -exclude".to_string());
        }

        let input = filter_input(input, &["idl"])?;

        if input.is_empty() {
            return Err("no .idl inputs".to_string());
        }

        for input in &input {
            fmt_idl(input, verbose)?;
        }

        return Ok(());
    }

    let input = filter_input(input, &["winmd", "idl"])?;

    if input.is_empty() {
        return Err("no inputs".to_string());
    }

    if output.is_empty() {
        return Err("no output".to_string());
    }

    let output_path = std::path::Path::new(&output).to_path_buf();

    let extension = match output_path.extension() {
        Some(extension) => extension.to_string_lossy().to_lowercase(),
        None => "".to_string(),
    };

    if !matches!(extension.as_str(), "idl" | "winmd") {
        return Err("output extension must be either .winmd or .idl".to_string());
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| format!("failed to create directory for `{output}`"))?;
    }

    let input = read_input(input, verbose)?;
    let reader = reader::Reader::new(&input);

    let buffer = if extension == "winmd" { write_output_winmd(reader, include, exclude)? } else { write_output_idl(reader, include, exclude)? };

    std::fs::write(&output_path, buffer).map_err(|_| format!("failed to write `{output}`"))?;
    let output_path = if !verbose && output_path.is_file() { output_path.file_name().unwrap().to_string_lossy().to_string() } else { canonicalize(&output_path)? };
    println!("  Finished writing `{}` in {:.2}s", display_path(&output_path), time.elapsed().as_secs_f32());
    Ok(())
}

fn filter_input(input: Vec<String>, filter: &[&str]) -> ToolResult<Vec<String>> {
    fn try_push(path: &std::path::Path, filter: &[&str], results: &mut Vec<String>) -> ToolResult<()> {
        if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();

            if filter.contains(&extension.as_str()) {
                results.push(canonicalize(path)?);
            }
        }

        Ok(())
    }

    let mut results = vec![];

    for input in &input {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            for entry in path.read_dir().map_err(|_| format!("failed to read directory `{input}`"))?.flatten() {
                let path = entry.path();

                if path.is_file() {
                    try_push(&path, filter, &mut results)?;
                }
            }
        } else {
            try_push(path, filter, &mut results)?;
        }
    }
    Ok(results)
}

fn canonicalize(path: &std::path::Path) -> ToolResult<String> {
    let path = std::fs::canonicalize(path).map_err(|_| format!("failed to find `{}`", path.display()))?;

    let path = if let Some(extension) = path.extension() {
        let extension = extension.to_string_lossy().to_lowercase();
        path.with_extension(extension)
    } else {
        path
    };

    Ok(path.to_string_lossy().to_string())
}

fn display_path(path: &str) -> String {
    path.trim_start_matches(r#"\\?\"#).to_string()
}

fn read_input(input: Vec<String>, verbose: bool) -> ToolResult<Vec<reader::File>> {
    let mut files = vec![];

    for input in &input {
        if input.ends_with("winmd") {
            if verbose {
                println!("   Include {}", display_path(input));
            }

            files.push(reader::File::new(input).map_err(|_| format!("failed to read `{}`", display_path(input)))?);
        } else {
            if verbose {
                println!("   Convert {}", display_path(input));
            }
            files.push(write_temp_winmd(input)?);
        }
    }

    Ok(files)
}

fn fmt_idl(input: &str, verbose: bool) -> ToolResult<()> {
    if verbose {
        println!("   Format {}", display_path(input));
    }
    // TODO:
    // 1. escape riddle keywords
    // 2. call rustfmt
    // 3. unescape riddle keywords (if step 2 succeeded)
    todo!()
}

fn write_temp_winmd(_input: &str) -> ToolResult<reader::File> {
    // TODO: parse .idl input and write an in-memory .winmd
    // The .winmd includes attributes pointing back to input file name, line, and column info.
    todo!()
}

fn write_output_winmd(_reader: reader::Reader, _include: Vec<String>, _exclude: Vec<String>) -> ToolResult<Vec<u8>> {
    // TODO: filter and validate metadata before writing final .winmd file.
    // Validation can use source file attributes if present to provide richer diagnostics.
    Ok(vec![])
}

fn write_output_idl(_reader: reader::Reader, _include: Vec<String>, _exclude: Vec<String>) -> ToolResult<Vec<u8>> {
    // TODO:
    // 1. filter and write final .idl file
    // 2. call rustfmt
    Ok(vec![])
}
