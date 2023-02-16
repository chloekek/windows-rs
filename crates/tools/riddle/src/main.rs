#![allow(dead_code)]

mod syntax;
mod sqlite;
use metadata::writer;
use std::io::Read;

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
        format!("{}\n  --> {}:{:?}:{:?} ", from.error.to_string(), from.filename, start.line, start.column)
    }
}

fn run() -> ToolResult<()> {
    let mut kind = ArgKind::None;
    let mut merge = Vec::<String>::new();
    let mut input = Vec::<String>::new();
    let mut reference = Vec::<String>::new();
    let mut output = String::new();
    let mut winrt = true;

    // let start = std::time::Instant::now();

    // let db = sqlite::Database::new().unwrap();
    // let files = metadata::reader::File::with_default(&[]).unwrap();
    // let reader = &metadata::reader::Reader::new(&files);
    // let root = reader.tree("Windows", &[]).unwrap();

    // for tree in root.flatten() {
    //     for def in reader.namespace_types(tree.namespace) {
    //         if reader.type_def_kind(def) == metadata::reader::TypeKind::Struct {
    //             let name = &format!("{}.{}", reader.type_def_namespace(def), reader.type_def_name(def));
    //             let architecture = reader.type_def_architecture(def);
    //             db.add_struct(name, architecture).unwrap();
    //         }
    //     }
    // }

    // println!("elapsed {}", start.elapsed().as_millis());

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

    // for merge in merge {
    //     // TODO: write the types in the winmd into `items`
    // }

    let input = load(input)?;
    let input = parse(input)?;
    let items = process(input)?;

    let buffer = writer::write("test", winrt, &items, &[]);
    std::fs::create_dir_all(std::path::Path::new(&output).parent().unwrap()).map_err(|_| format!("failed to create directory for `{output}`"))?;
    std::fs::write(&output, buffer).map_err(|_| format!("failed to write `{output}`"))
}

fn load(input: Vec<String>) -> ToolResult<Vec<(String, String)>> {
    let mut result = vec![];

    for filename in input {
        let mut file = std::fs::File::open(&filename).map_err(|_| format!("failed to open `{filename}`"))?;
        let filename = std::fs::canonicalize(&filename).map_err(|_| format!("failed to canonicalize `{filename}`"))?;
        let filename = filename.to_string_lossy().trim_start_matches(r#"\\?\"#).to_string();
        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|_| format!("failed to read `{filename}`"))?;
        result.push((filename, source));
    }

    Ok(result)
}

fn parse(input: Vec<(String, String)>) -> SyntaxResult<Vec<(String, syntax::File)>> {
    let mut result = vec![];
    for (filename, source) in input {
        match syn::parse_str::<syntax::File>(&source) {
            Ok(file) => result.push((filename, file)),
            Err(error) => return Err(SyntaxError::new(filename, error)),
        }
    }
    Ok(result)
}

fn process(input: Vec<(String, syntax::File)>) -> SyntaxResult<Vec<writer::Item>> {
    // TODO: need to deal with references across files and from winmd files
    let mut items = vec![];
    for (filename, file) in input {
        file.to_writer(&mut items).map_err(|error| SyntaxError::new(filename, error))?;
    }
    Ok(items)
}

// TODO: remove -merge and just include winmd files from -input.

fn print_help() -> ToolResult<()> {
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
