use std::fs;

fn main() {
    let env_file_content = fs::read_to_string("../.env")
        .expect("could not find .env file");

    for line in env_file_content.lines() {
        let line = line.replace('"', "");
        if let Some((k, v)) = line.split_once('=') {
            println!("cargo:rustc-env={k}={v}");
        }
    }

    tauri_build::build()
}
