use super::*;
use std::io::Write;

pub fn format(idl: &str) -> Option<String> {
    // TODO:
    // 1. escape riddle keywords
    // 2. call rustfmt
    // 3. unescape riddle keywords (if step 2 succeeded)

    let Ok(mut child) = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn() else {
        return None;
    };
    let Some(mut stdin) = child.stdin.take() else {
        return None;
    };
    if stdin.write_all(idl.as_bytes()).is_err() {
        return None;
    };
    drop(stdin);
    let Ok(output) = child.wait_with_output() else {
        return None;
    };
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

pub fn write(reader: &reader::Reader, filter: &reader::Filter) -> ToolResult<Vec<u8>> {
    let buffer = tree_to_idl(reader, "", &reader.tree("", filter), filter).to_string();
    Ok(format(buffer.as_str()).expect("format failed").into_bytes())
}

fn tree_to_idl(reader: &reader::Reader, name: &str, tree: &reader::Tree, filter: &reader::Filter) -> proc_macro2::TokenStream {
    if name.is_empty() {
        let nested = tree.nested.iter().map(|(name, tree)| tree_to_idl(reader, name, tree, filter));
        quote::quote! { #(#nested)* }
    } else if filter.includes_namespace(tree.namespace) {
        let name = to_ident(name);
        let nested = tree.nested.iter().map(|(name, tree)| tree_to_idl(reader, name, tree, filter));
        let types = reader.namespace_types(tree.namespace, filter).map(|ty| type_to_idl(reader, ty));

        quote::quote! {
            mod #name {
                #(#nested)*
                #(#types)*
            }
        }
    } else {
        quote::quote! {}
    }
}

fn type_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    match reader.type_def_kind(ty) {
        reader::TypeKind::Class => class_to_idl(reader, ty),
        reader::TypeKind::Interface => interface_to_idl(reader, ty),
        reader::TypeKind::Enum => enum_to_idl(reader, ty),
        reader::TypeKind::Struct => struct_to_idl(reader, ty),
        reader::TypeKind::Delegate => delegate_to_idl(reader, ty),
    }
}

fn class_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    let name = to_ident(reader.type_def_name(ty));

    quote::quote! {
        struct #name {}
    }
}

fn interface_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    let name = to_ident(reader.type_def_name(ty));

    quote::quote! {
        struct #name {}
    }
}

fn enum_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    let name = to_ident(reader.type_def_name(ty));

    quote::quote! {
        struct #name {}
    }
}

fn struct_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    let name = to_ident(reader.type_def_name(ty));

    quote::quote! {
        struct #name {}
    }
}

fn delegate_to_idl(reader: &reader::Reader, ty: reader::TypeDef) -> proc_macro2::TokenStream {
    let name = to_ident(reader.type_def_name(ty));

    quote::quote! {
        struct #name {}
    }
}

fn to_ident(name: &str) -> proc_macro2::Ident {
    quote::format_ident!("{}", reader::trim_tick(name))
}
