use std::{env, fs, io};
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;

const DOWNLOAD_PATH: &str = "generator/antlr4-4.8-2-SNAPSHOT-complete.jar";
const DOWNLOAD_URL: &str = "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar";

fn download_antlr() {
    let path = Path::new(DOWNLOAD_PATH);
    if path.exists() {
        return;
    }

    if let Some(parent) = Path::new(DOWNLOAD_PATH).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let file = File::create(DOWNLOAD_PATH).unwrap();
    let mut writer = io::BufWriter::new(file);

    let mut reader = ureq::get(DOWNLOAD_URL)
        .call()
        .unwrap()
        .into_reader();
    io::copy(&mut reader, &mut writer).unwrap();
}

fn main() {
    download_antlr();

    let grammar_files = glob::glob("grammar/**/*.g4").unwrap();

    for path in grammar_files {
        let grammar = path.unwrap();
        let _ = gen_for_grammar(&*grammar, Path::new(DOWNLOAD_PATH));
    }

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=/home/rrevenantt/dev/antlr4/tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_grammar(
    grammar_path: &Path,
    antlr_path: &Path,
) -> Result<(), Box<dyn Error>> {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir);
    let relative_path = grammar_path.strip_prefix("grammar").unwrap();
    let rust_relative_path = relative_path.with_file_name(relative_path.file_name().unwrap().to_ascii_lowercase()).with_extension("");
    let rust_path = env::current_dir().unwrap().join("src").join(rust_relative_path);

    let antlr_path = env::current_dir().unwrap().join(antlr_path);

    let input = grammar_path.parent().unwrap();
    let c = Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg(rust_path)
        .arg(grammar_path.file_name().unwrap())
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    println!("cargo:rerun-if-changed={}", grammar_path.display());
    Ok(())
}