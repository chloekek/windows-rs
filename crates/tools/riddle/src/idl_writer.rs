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

pub fn write(_reader: &reader::Reader, tree: &reader::Tree) -> ToolResult<Vec<u8>> {
    let buffer = tree_to_idl("", tree).to_string();
    Ok(format(buffer.as_str()).expect("format failed").into_bytes())
}

fn tree_to_idl(name: &str, tree: &reader::Tree) -> proc_macro2::TokenStream {
    let nested = tree.nested.iter().map(|(name, tree)| tree_to_idl(name, tree));

    if name.is_empty() {
        quote::quote! { #(#nested)* }
    } else {
        let name = quote::format_ident!("{}", name);
        quote::quote! {
            mod #name {
                #(#nested)*
            }
        }
    }

}