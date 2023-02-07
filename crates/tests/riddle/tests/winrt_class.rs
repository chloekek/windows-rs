use windows_metadata::reader::*;

#[test]
fn test() {
    let output = test_riddle::winrt_riddle("tests/winrt_class.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Class);
}
