extern crate core;

use crate::syntax_tree::syntax_tree_node::SyntaxTreeNode;
use crate::tokenizer::token::Token;
use crate::transpiler::prettifier::Prettifier;
use std::fs;
use std::path::Path;

pub mod syntax_tree;
pub mod tokenizer;
pub mod transpiler;

fn main() {
    let input_file_path: &Path = Path::new("./src/input/source.cs");
    let contents: String =
        fs::read_to_string(input_file_path)
            .expect("Should have been able to read the file");

    let source_code_tokens: Vec<Token> = tokenizer::tokenizer::tokenize(contents)
        .expect("Failed to tokenize source code");

    let abstract_syntax_tree: SyntaxTreeNode = syntax_tree::syntax_tree_builder::parse(source_code_tokens)
        .expect("Failed to parse abstract syntax tree");

    let transpiled_code: String = transpiler::transpiler::transpile(abstract_syntax_tree);

    let prettifier: Prettifier = Prettifier::new(' ', 4);
    let pretty_transpiled_code: String = prettifier.prettify(transpiled_code);

    let output_file_path: &Path = Path::new("./src/output/result.clj");

    if let Some(directory_path) = output_file_path.parent() {
        fs::create_dir_all(directory_path)
            .expect("Should have been able to create the directory");
    }

    fs::write(output_file_path, pretty_transpiled_code)
        .expect("Should have been able to write output file");
}
