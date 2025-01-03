use crate::syntax_tree::syntax_tree_node::{SyntaxTreeNode, SyntaxTreeNodeType};

pub fn transpile(abstract_syntax_tree: SyntaxTreeNode) -> String {
    let mut output: String = String::new();

    output.push_str(&convert_abstract_syntax_tree_to_code(&abstract_syntax_tree));

    for child in abstract_syntax_tree.children {
        output.push_str(&convert_abstract_syntax_tree_to_code(&child));
    }

    output
}

fn convert_abstract_syntax_tree_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    match syntax_tree_node.type_ {
        SyntaxTreeNodeType::Namespace => {
            convert_namespace_syntax_tree_node_to_code(syntax_tree_node)
        }
        SyntaxTreeNodeType::Method => convert_method_syntax_tree_node_to_code(syntax_tree_node),
        SyntaxTreeNodeType::Literal => convert_literal_syntax_tree_node_to_code(syntax_tree_node),
        SyntaxTreeNodeType::Expression => {
            convert_expression_syntax_tree_node_to_code(syntax_tree_node)
        }
        SyntaxTreeNodeType::Assignment => {
            convert_assignment_syntax_tree_node_to_code(syntax_tree_node)
        }
        SyntaxTreeNodeType::EqualityCheck => {
            convert_expression_syntax_tree_node_to_code(syntax_tree_node)
        }
        SyntaxTreeNodeType::Branch => convert_branch_syntax_tree_node_to_code(syntax_tree_node),
        SyntaxTreeNodeType::Comment => convert_comment_syntax_tree_node_to_code(syntax_tree_node),
        SyntaxTreeNodeType::Collection => {
            convert_collection_syntax_tree_node_to_code(syntax_tree_node)
        }
        _ => panic!(
            "Unable to convert abstract syntax tree node {} to code",
            &syntax_tree_node.type_
        ),
    }
}

fn convert_namespace_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    format!("(ns {})\n\n", syntax_tree_node.value.clone().unwrap())
}

fn convert_method_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::new();

    output.push_str("(defn ");
    output.push_str(syntax_tree_node.value.clone().unwrap().as_str());
    output.push_str(" [");
    output.push_str(
        syntax_tree_node
            .children
            .iter()
            .filter(|token| matches!(token.type_, SyntaxTreeNodeType::MethodArgument))
            .map(|token| token.value.clone().unwrap())
            .collect::<Vec<String>>()
            .join(" ")
            .as_str(),
    );
    output.push_str("]\n");
    output.push_str(
        syntax_tree_node
            .children
            .iter()
            .filter(|child| !matches!(child.type_, SyntaxTreeNodeType::MethodArgument))
            .map(convert_abstract_syntax_tree_to_code)
            .collect::<Vec<String>>()
            .join("\n")
            .as_str(),
    );

    let number_of_open_parens = output.chars().filter(|c| *c == '(').count();
    let number_of_close_parens = output.chars().filter(|c| *c == ')').count();
    output.push_str(
        ")".repeat(number_of_open_parens - number_of_close_parens)
            .as_str(),
    );

    output.push_str("\n\n");

    output
}

fn convert_expression_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::new();

    output.push_str("(");
    output.push_str(syntax_tree_node.value.clone().unwrap().as_str());
    output.push_str(" ");
    output.push_str(
        syntax_tree_node
            .children
            .iter()
            .map(convert_abstract_syntax_tree_to_code)
            .collect::<Vec<String>>()
            .join(" ")
            .as_str(),
    );
    output.push_str(")");

    output
}

fn convert_assignment_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::from("(let [");

    if syntax_tree_node
        .children
        .iter()
        .any(|child| matches!(child.type_, SyntaxTreeNodeType::Assignment))
    {
        output.push_str(
            syntax_tree_node
                .children
                .iter()
                .map(|child| {
                    String::from(
                        child.value.clone().unwrap()
                            + " "
                            + convert_abstract_syntax_tree_to_code(&child.children[0]).as_str(),
                    )
                })
                .collect::<Vec<String>>()
                .join("\n  ")
                .as_str(),
        );
    } else {
        output.push_str(syntax_tree_node.value.clone().unwrap().as_str());
        output.push_str(" ");
        output
            .push_str(convert_abstract_syntax_tree_to_code(&syntax_tree_node.children[0]).as_str());
    }

    output.push_str("]");

    output
}

fn convert_literal_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    match &syntax_tree_node.value {
        Some(s) if s == "null" => String::from("nil"),
        Some(s) => s.clone(),
        None => panic!("literal has no value"),
    }
}

fn convert_branch_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::new();
    let mut index_offset: usize = 0;

    if syntax_tree_node.value.clone().unwrap() == "if" {
        index_offset = 1;
        output.push_str("(if ");
        output.push_str(&convert_abstract_syntax_tree_to_code(
            &syntax_tree_node.children[0],
        ));
        output.push('\n');
    }

    if syntax_tree_node.children.len() == index_offset + 1 {
        output.push_str(
            convert_abstract_syntax_tree_to_code(&syntax_tree_node.children[index_offset]).as_str(),
        );
    } else {
        output.push_str("(do \n");
        output.push_str(
            syntax_tree_node
                .children
                .iter()
                .skip(index_offset)
                .map(convert_abstract_syntax_tree_to_code)
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
        output.push(')');
    }

    let number_of_open_parens: i32 = output.chars().filter(|c| *c == '(').count() as i32;
    let number_of_close_parens: i32 = output.chars().filter(|c| *c == ')').count() as i32;
    let number_of_missing_parens: i32 = number_of_open_parens - number_of_close_parens;

    if number_of_missing_parens - 1 >= 0 {
        output.push_str(")".repeat((number_of_missing_parens - 1) as usize).as_str());
    }

    output
}

fn convert_comment_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::from(";;");
    output.push_str(&syntax_tree_node.value.clone().unwrap().as_str());
    output
}

fn convert_collection_syntax_tree_node_to_code(syntax_tree_node: &SyntaxTreeNode) -> String {
    let mut output: String = String::from("[]");

    let collection_body: String = syntax_tree_node
        .children
        .iter()
        .map(convert_abstract_syntax_tree_to_code)
        .collect::<Vec<String>>()
        .join(" ");

    output.insert_str(1, collection_body.as_str());

    output
}
