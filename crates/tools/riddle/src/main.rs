mod syntax;
use std::io::Read;

// TODO: need to do all the linting/type inspection/attribute reflection here and just use windows-metadata for writing the items out
// where that library does minimal validation since the syntax spans won't be known at that point.

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

type ToolResult = std::result::Result<(), String>;

fn run() -> ToolResult {
    let mut kind = ArgKind::None;
    let mut merge = Vec::<String>::new();
    let mut input = Vec::<String>::new();
    let mut reference = Vec::<String>::new();
    let mut output = String::new();
    let mut winrt = true;

    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }
        match kind {
            ArgKind::None => match arg.as_str() {
                "-merge" => kind = ArgKind::Merge,
                "-input" => kind = ArgKind::Input,
                "-ref" => kind = ArgKind::Reference,
                "-output" => kind = ArgKind::Output,
                "-win32" => {
                    winrt = false;
                    kind = ArgKind::None;
                }
                _ => print_help()?,
            },
            ArgKind::Merge => merge.push(arg),
            ArgKind::Input => input.push(arg),
            ArgKind::Reference => reference.push(arg),
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg;
                } else {
                    print_help()?;
                }
            }
        }
    }

    if merge.len() + input.len() == 0 || output.is_empty() {
        print_help()?;
    }

    let mut items = Vec::new();

    // for merge in merge {
    //     // TODO: write the types in the winmd into `items`
    // }

    for filename in &input {
        let mut file = std::fs::File::open(filename).map_err(|_| format!("failed to open `{filename}`"))?;
        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|_| format!("failed to read `{filename}`"))?;

        // TODO: need an "analysis" pass between parsing and writing the items. That's the point at which the syntax tree
        // is fully-formed and we still have the span information for error reporting but before the low-level items are
        // pushed to the metadata writer for winmd generation. 

        // TODO: remove all the to_writer methods frm the syntax module - just gets teh parsed `File` and then do all that here along with resolving names
        // and analysis. That way we can also do cross-file references e.g. windows.ui.idl refers to windows.foundation.idl

        if let Err(error) = syn::parse_str::<syntax::File>(&source).and_then(|file|file.normalize()).and_then(|file| file.to_writer(&mut items)) {
            let start = error.span().start();
            let filename = std::fs::canonicalize(filename).map_err(|_| format!("failed to canonicalize `{filename}`"))?;
            return Err(format!("{error}\n  --> {}:{:?}:{:?} ", filename.to_string_lossy().trim_start_matches(r#"\\?\"#), start.line, start.column));
        }
    }

    let buffer = metadata::writer::write("test", winrt, &items, &[]);
    std::fs::create_dir_all(std::path::Path::new(&output).parent().unwrap()).map_err(|_| format!("failed to create directory for `{output}`"))?;
    std::fs::write(&output, buffer).map_err(|_| format!("failed to write `{output}`"))
}

fn print_help() -> ToolResult {
    Err(r#"riddle.exe [options...]

options:
  -input  <path>  Path to source file with type definitions to parse
  -merge  <path>  Path to file or directory containing .winmd files to merge
  -ref    <path>  Path to file or directory containing .winmd files to reference
  -output <path>  Path to .winmd file to generate
  -win32          Kind of .winmd to generate; default is WinRT

examples:
  riddle -input first.rs second.rs -output out.winmd -ref local
  riddle -merge first.winmd second.winmd -output out.winmd
"#
    .to_string())
}

enum ArgKind {
    None,
    Merge,
    Input,
    Reference,
    Output,
}
