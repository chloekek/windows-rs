use test_riddle::run_riddle;
use windows_metadata::reader::*;

#[test]
fn riddle_interface() {
    let output = run_riddle("tests/interface.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Interface);

    let methods: Vec<MethodDef> = reader.type_def_methods(def).collect();
    assert_eq!(methods.len(), 2);

    let signature = reader.method_def_signature(methods[0], &[]);
    assert_eq!(reader.method_def_name(signature.def), "A");
    assert!(signature.return_type.is_none());
    assert!(signature.params.is_empty());

    let signature = reader.method_def_signature(methods[1], &[]);
    assert_eq!(reader.method_def_name(signature.def), "B");
    assert_eq!(signature.return_type, Some(Type::I32));
    assert!(signature.params.is_empty());
}
