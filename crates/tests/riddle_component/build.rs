use std::fs::*;
use std::process::*;

fn main() {
    let mut command = Command::new("cargo.exe");
    command.arg("install").arg("--path").arg("../../tools/riddle");

    if !command.status().unwrap().success() {
        panic!("Failed to install riddle");
    }

    let mut command = Command::new("riddle.exe");
    command.arg("-input").arg("src/component.idl").arg("-output").arg("component.winmd");

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }

    let files = metadata::reader::File::with_default(&["component.winmd"]).expect("Failed to read metadata");
    write("src/bindings.rs", bindgen::component("Component", &files)).expect("Failed to write bindings");
    let _ = Command::new("rustfmt").arg("src/bindings.rs").status();

}