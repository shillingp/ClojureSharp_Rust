extern crate core;

use crate::syntax_tree::syntax_tree_node::SyntaxTreeNode;
use crate::tokenizer::token::Token;
use std::fs;
use std::path::Path;

pub mod tokenizer;
pub mod syntax_tree;
pub mod transpiler;

fn main() {
    let input_file_path = Path::new("./src/input/source.cs");
    let contents: String = fs::read_to_string(input_file_path)
        .expect("Should have been able to read the file");

    let source_code_tokens: Vec<Token> = tokenizer::tokenizer::tokenize(contents);

    let abstract_syntax_tree: Result<SyntaxTreeNode, String> = syntax_tree::syntax_tree_builder::parse(source_code_tokens);

    let transpiled_code: String = transpiler::transpiler::transpile(abstract_syntax_tree
        .expect("Failed to parse abstract syntax tree"));

    let prettifier = transpiler::prettifier::Prettifier::new(' ', 4);
    let pretty_transpiled_code: String = prettifier.prettify(transpiled_code);

    let output_file_path = Path::new("./src/output/result.clj");
    fs::write(output_file_path, pretty_transpiled_code)
        .expect("Should have been able to write output file");
}
