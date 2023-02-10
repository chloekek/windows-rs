use windows_metadata::reader::*;

#[test]
fn test() {
    let output = test_riddle::winrt_riddle("tests/winrt_params.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Interface);

    let methods: Vec<MethodDef> = reader.type_def_methods(def).collect();
    assert_eq!(methods.len(), 3);

    let signature = reader.method_def_signature(methods[0], &[]);
    assert_eq!(reader.method_def_name(signature.def), "A");
    assert_eq!(signature.return_type, Some(Type::Bool));
    assert_eq!(signature.params.len(), 2);
    assert_eq!(reader.param_name(signature.params[0].def), "a");
    assert_eq!(signature.params[0].ty, Type::Bool);
    assert_eq!(reader.param_name(signature.params[1].def), "b");
    //assert_eq!(signature.params[1].ty, Type::ByRef(Type::Bool));
}
