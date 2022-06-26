use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use tree_sitter;
use tree_sitter_cli::generate;

use tempdir::TempDir;

fn main() {
    let dir = TempDir::new("grammar").unwrap();
    let grammar_file = dir.path().join("parser.c");
    let mut f = File::create(grammar_file).unwrap();

    let grammar = rust_sitter_tool::generate_grammar(&PathBuf::from("src/main.rs")).unwrap();

    f.write_all(generate::generate_parser_for_grammar(&grammar).unwrap().1.as_bytes()).unwrap();
    drop(f);

    let header_dir: PathBuf = dir.path().join("tree_sitter");
    std::fs::create_dir(&header_dir).unwrap();
    let mut parser_file = File::create(header_dir.join("parser.h")).unwrap();
    parser_file.write_all(tree_sitter::PARSER_HEADER.as_bytes()).unwrap();
    drop(parser_file);

    cc::Build::new()
        .include(&dir)
        .file(dir.path().join("parser.c"))
        .compile("grammar");
}