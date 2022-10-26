use std::fs;

use cierra::taurus::Taurus;
use insta::assert_debug_snapshot;

#[test]
fn must_parse() {
    let test_files = glob::glob("tests/parser/*.c").unwrap();
    for file in test_files {
        let path = file.unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let ast = Taurus::parse(&content);
        assert_debug_snapshot!(name, ast);
    }
}
