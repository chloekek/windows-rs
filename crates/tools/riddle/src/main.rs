#![allow(dead_code)]

mod syntax;
use metadata::{reader, writer};

// TODO: need to do all the linting/type inspection/attribute reflection here and just use windows-metadata for writing the items out
// where that library does minimal validation since the syntax spans won't be known at that point.

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

struct SyntaxError {
    filename: String,
    error: syn::Error,
}

impl SyntaxError {
    fn new(filename: String, error: syn::Error) -> Self {
        Self { filename, error }
    }
}

type ToolResult<T> = std::result::Result<T, String>;
type SyntaxResult<T> = std::result::Result<T, SyntaxError>;

impl std::convert::From<SyntaxError> for String {
    fn from(from: SyntaxError) -> Self {
        let start = from.error.span().start();
        format!("{}\n  --> {}:{:?}:{:?} ", from.error, from.filename, start.line, start.column)
    }
}

enum ArgKind {
    None,
    Input,
    Reference,
    Output,
}

// 1. convert all idl files into winmd files
//  - don't check references (ignore value/class type?)
//  - add attributes for source file/line number for error reporting
// 2. load all winmd files into Reader
//  - this avoids needing a relational database like sqlite
// 3. build in-memory model from Reader with all relevant info and validate
//  - this will also help improve the Reader to be more robust and resilient
//  - as it has to deal with faulty winmd files
// 4. write out merged winmd file from in-memory model (e.g. Items) with metadata writer

fn run() -> ToolResult<()> {
    let start = std::time::Instant::now();

    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return Err(r#"riddle.exe [options...]

options:
  -in  <path>  Path to file or directory containing .winmd and/or .idl files
  -ref <path>  Path to file or directory containing .winmd files to reference
  -out <path>  Path to .winmd file to generate
  -verbose     Show detailed progress information

examples:
  riddle -in first.idl second.idl -ref ref.winmd -out out.winmd
  riddle -in first.idl second.winmd -out out.winmd
"#
        .to_string());
    }

    let mut kind = ArgKind::None;
    let mut input = Vec::<String>::new();
    let mut reference = Vec::<String>::new();
    let mut output = String::new();
    let mut verbose = false;

    for arg in args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }
        match kind {
            ArgKind::None => match arg.as_str() {
                "-in" => kind = ArgKind::Input,
                "-ref" => kind = ArgKind::Reference,
                "-out" => kind = ArgKind::Output,
                "-verbose" => verbose = true,
                _ => return Err("invalid option".to_string()),
            },
            ArgKind::Input => input.push(arg),
            ArgKind::Reference => reference.push(arg),
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg;
                } else {
                    return Err("too many outputs".to_string());
                }
            }
        }
    }

    let input = filter_input(input, &["winmd", "idl"])?;
    let reference = filter_input(reference, &["winmd"])?;

    if input.is_empty() {
        return Err("no inputs".to_string());
    }

    if output.is_empty() {
        return Err("no output".to_string());
    }

    let mut output_path = std::path::Path::new(&output).to_path_buf();
    if output_path.extension().is_none() {
        output_path = output_path.with_extension("winmd");
    }
    std::fs::create_dir_all(output_path.parent().unwrap()).map_err(|_| format!("failed to create directory for `{output}`"))?;

    let files = load_input(input, reference, verbose)?;
    let _reader = reader::Reader::new(&files);

    // TODO: now read each typedef in the reader (that has the include bit set) and build the set of output records
    // to pass to the writer, using the reader to perform validation on each type.

    let buffer = writer::write("test", true, &[], &[]);
    std::fs::write(&output_path, buffer).map_err(|_| format!("failed to write `{output}`"))?;
    
    let output_path = if verbose { canonicalize(&output_path, "winmd")? } else { output_path.file_name().unwrap().to_string_lossy().to_string() };
    println!("  Finished {} in {:.2}s", display_path(&output_path), start.elapsed().as_secs_f32());
    Ok(())
}

fn canonicalize(path: &std::path::Path, extension: &str) -> ToolResult<String> {
    let path = std::fs::canonicalize(path).map_err(|_| format!("failed to find `{}`", path.display()))?;
    Ok(path.with_extension(extension).to_string_lossy().to_string())
}

fn display_path(path: &str) -> String {
    path.trim_start_matches(r#"\\?\"#).to_string()
}

fn filter_input(input: Vec<String>, filter: &[&str]) -> ToolResult<Vec<String>> {
    fn try_push(path: &std::path::Path, filter: &[&str], results: &mut Vec<String>) -> ToolResult<()> {
        if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            if filter.contains(&extension.as_str()) {
                results.push(canonicalize(&path, &extension)?);
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

fn load_input(input: Vec<String>, reference: Vec<String>, verbose: bool) -> ToolResult<Vec<reader::File>> {
    let mut results = vec![];

    for input in &input {
            if input.ends_with("winmd") {
                if verbose {
                    println!("   Include {}", display_path(input));
                }
                results.push(reader::File::new(input).map_err(|_| format!("failed to read `{}`", display_path(input)))?);
            } else {
                if verbose {
                    println!("   Convert {}", display_path(input));
                }
                results.push(convert_input(input)?);
            }
        }

    for reference in &reference {
        if verbose {
            println!(" Reference {}", display_path(reference));
        }
        results.push(reader::File::new(reference).map_err(|_| format!("failed to read `{}`", display_path(reference)))?);
    }

    Ok(results)
}

fn convert_input(_input: &str) -> ToolResult<reader::File> {
    // TODO: parse without chasing references and form a single winmd per IDL
    let buffer = writer::write("test", true, &[], &[]);
    Ok(reader::File::from_buffer(buffer).expect("writer produced invalid winmd"))
}

// fn load(input: Vec<String>) -> ToolResult<Vec<(String, String)>> {
//     let mut result = vec![];

//     for filename in input {
//         let mut file = std::fs::File::open(&filename).map_err(|_| format!("failed to open `{filename}`"))?;
//         let filename = std::fs::canonicalize(&filename).map_err(|_| format!("failed to canonicalize `{filename}`"))?;
//         let filename = filename.to_string_lossy().trim_start_matches(r#"\\?\"#).to_string();
//         let mut source = String::new();
//         file.read_to_string(&mut source).map_err(|_| format!("failed to read `{filename}`"))?;
//         result.push((filename, source));
//     }

//     Ok(result)
// }

// fn parse(input: Vec<(String, String)>) -> SyntaxResult<Vec<(String, syntax::File)>> {
//     let mut result = vec![];
//     for (filename, source) in input {
//         match syn::parse_str::<syntax::File>(&source) {
//             Ok(file) => result.push((filename, file)),
//             Err(error) => return Err(SyntaxError::new(filename, error)),
//         }
//     }
//     Ok(result)
// }

// fn process(input: Vec<(String, syntax::File)>) -> SyntaxResult<Vec<writer::Item>> {
//     // TODO: need to deal with references across files and from winmd files
//     let mut items = vec![];
//     for (filename, file) in input {
//         file.to_writer(&mut items).map_err(|error| SyntaxError::new(filename, error))?;
//     }
//     Ok(items)
// }
