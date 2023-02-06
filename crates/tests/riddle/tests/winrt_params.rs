use windows_metadata::reader::*;

#[test]
fn riddle_interface() {
    let output = test_riddle::winrt_riddle("tests/winrt_params.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Interface);

    let methods: Vec<MethodDef> = reader.type_def_methods(def).collect();
    assert_eq!(methods.len(), 1);

    let signature = reader.method_def_signature(methods[0], &[]);
    assert_eq!(reader.method_def_name(signature.def), "P1");
    assert_eq!(signature.return_type, Some(Type::Bool));
    assert_eq!(signature.params.len(), 2);
}
