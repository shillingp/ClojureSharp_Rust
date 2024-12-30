use crate::syntax_tree::syntax_tree_node::{SyntaxTreeNode, SyntaxTreeNodeType};
use crate::tokenizer::token::{Token, TokenType};

pub(crate) fn parse(source_tokens: Vec<Token>) -> Result<SyntaxTreeNode, String> {
    if !matches!(source_tokens[0].type_, TokenType::NamespaceToken)
        || !matches!(source_tokens[1].type_, TokenType::NameIdentifierToken) {
        return Err(String::from("sd"));
    }

    let mut current_index: usize = 0;
    let mut internal_nodes: Vec<SyntaxTreeNode> = Vec::new();

    while current_index < source_tokens.len() as usize {
        match (source_tokens.iter().nth(current_index),
               source_tokens.iter().nth(current_index + 1),
               source_tokens.iter().nth(current_index + 2)) {
            (Some(first), Some(second), Some(third))
            if matches!(first.type_, TokenType::TypeDeclarationToken)
                && matches!(second.type_, TokenType::NameIdentifierToken)
                && matches!(third.type_, TokenType::OpenParenthesisToken) => {
                    let end_index = find_index_of_last_closing_scope(&source_tokens, current_index)
                        .expect("scope end location not found");

                    internal_nodes.push(parse_method(&source_tokens[current_index..=end_index]));
                    current_index = end_index;
                }
            (Some(first), Some(second), _)
            if matches!(first.type_, TokenType::ClassToken)
                && matches!(second.type_, TokenType::NameIdentifierToken) => {
                todo!()
            },
            (Some(first), Some(second), _)
            if matches!(first.type_, TokenType::NameIdentifierToken)
                && matches!(second.type_, TokenType::OpenParenthesisToken) => {
                let semicolon_index = current_index + source_tokens.iter().skip(current_index)
                    .position(|token| matches!(token.type_, TokenType::SemicolonToken))
                    .expect("semicolon location not found");

                internal_nodes.push(parse_expression(&source_tokens[current_index..semicolon_index]));
                current_index = semicolon_index;
            },
            _ => current_index += 1,
        }
    }

    Ok(SyntaxTreeNode {
        value: source_tokens[1].value.clone(),
        type_: SyntaxTreeNodeType::Namespace,
        children: internal_nodes
    })
}

fn find_index_of_last_closing_scope(source_tokens: &[Token], index_to_start: usize) -> Option<usize> {
    let mut open_scope_count: u32 = 0;

    for (index, token) in source_tokens.iter().skip(index_to_start).enumerate() {
        match token.type_ {
            TokenType::CloseScopeToken => {
                open_scope_count -= 1;
                if open_scope_count == 0 {
                    return Some(index + index_to_start);
                }
            }
            TokenType::OpenScopeToken => {
                open_scope_count += 1;
            }
            _ => {}
        };
    }

    None
}

fn parse_method(method_tokens: &[Token]) -> SyntaxTreeNode {
    let argument_open_parenthesis_index: usize = method_tokens.iter()
        .position(|token| matches!(token.type_, TokenType::OpenParenthesisToken))
        .unwrap();
    let argument_close_parenthesis_index: usize = method_tokens.iter()
        .position(|token| matches!(token.type_, TokenType::CloseParenthesisToken))
        .unwrap();
    let method_argument_tokens: &[Token] = &method_tokens[(argument_open_parenthesis_index+1)..argument_close_parenthesis_index];

    let method_scope_open_index: usize = method_tokens.iter()
        .position(|token| matches!(token.type_, TokenType::OpenScopeToken))
        .unwrap();
    let method_body_tokens: &[Token] = &method_tokens[method_scope_open_index+1..];

    let mut method_nodes = Vec::new();
    method_nodes.extend(parse_method_arguments(method_argument_tokens));
    method_nodes.extend(parse_internal_scope(method_body_tokens));

    SyntaxTreeNode {
        value: method_tokens[1].value.clone(),
        type_: SyntaxTreeNodeType::Method,
        children: method_nodes
    }
}

fn parse_method_arguments(argument_tokens: &[Token]) -> Vec<SyntaxTreeNode> {
    argument_tokens.iter()
        .filter(|token| matches!(token.type_, TokenType::NameIdentifierToken))
        .map(|token| SyntaxTreeNode {
            value: token.value.clone(),
            type_: SyntaxTreeNodeType::MethodArgument,
            children: Vec::new(),
        })
        .collect::<Vec<SyntaxTreeNode>>()
}

fn parse_internal_scope(internal_tokens: &[Token]) -> Vec<SyntaxTreeNode> {
    let mut scope_nodes: Vec<SyntaxTreeNode> = Vec::new();

    let mut token_index: usize = 0;
    while token_index < internal_tokens.len() {
        let token = &internal_tokens[token_index];

        match token.type_ {
            TokenType::ReturnToken | TokenType::CloseScopeToken => {
                token_index += 1;
            },
            TokenType::CommentToken => {
                scope_nodes.push(SyntaxTreeNode {
                    value: token.value.clone(),
                    type_: SyntaxTreeNodeType::Comment,
                    children: Vec::new(),
                });
                token_index += 1;
            }
            _ => {
                let mut end_of_scope_index: usize = token_index + internal_tokens
                    .iter().skip(token_index)
                    .position(|token| matches!(token.type_, TokenType::SemicolonToken))
                    .unwrap_or(0);

                let open_scope_index: Option<usize> = internal_tokens
                    .iter().skip(token_index)
                    .position(|token| matches!(token.type_, TokenType::OpenScopeToken));

                if open_scope_index.is_some_and(|open| (token_index + open) < end_of_scope_index) {
                    end_of_scope_index = find_index_of_last_closing_scope(&internal_tokens, token_index).unwrap();
                }

                scope_nodes.push(parse_expression(&internal_tokens[token_index..=end_of_scope_index]));

                token_index = end_of_scope_index + 1;
            }
        }
    }

    group_consecutive_assignments(scope_nodes)

    // scope_nodes
}

fn group_consecutive_assignments(body_nodes: Vec<SyntaxTreeNode>) -> Vec<SyntaxTreeNode> {
    let mut body_nodes_with_compound_assignments: Vec<SyntaxTreeNode> = Vec::new();

    let mut i: usize = 0;
    while i < body_nodes.len() {
        if !matches!(body_nodes[i].type_, SyntaxTreeNodeType::Assignment) {
            body_nodes_with_compound_assignments.push(body_nodes[i].clone());
            i += 1;
            continue;
        }

        let mut j: usize = i + 1;
        while j < body_nodes.len() && matches!(body_nodes[j].type_, SyntaxTreeNodeType::Assignment) {
            j += 1;
        }

        body_nodes_with_compound_assignments.push(if i + 1 == j {
            body_nodes[i].clone()
        } else {
            SyntaxTreeNode {
                value: None,
                type_: SyntaxTreeNodeType::Assignment,
                children: Vec::from(&body_nodes[i..j]),
            }
        });

        i = j;
    }

    body_nodes_with_compound_assignments
}

fn parse_expression(expression_tokens: &[Token]) -> SyntaxTreeNode {
    let expression_tokens = match expression_tokens.last().unwrap().type_ {
        TokenType::SemicolonToken => &expression_tokens[..(expression_tokens.len()-1)],
        _ => expression_tokens
    };

    if expression_tokens.len() == 1 {
        return SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Literal,
            children: Vec::new(),
        }
    }

    if matches!(expression_tokens[0].type_, TokenType::ReturnToken) {
        return parse_expression(&expression_tokens[1..]);
    }

    if matches!(expression_tokens[0].type_, TokenType::OpenParenthesisToken) {
        if let Some(close_paren_index) = expression_tokens.iter()
            .rposition(|token| matches!(token.type_, TokenType::CloseParenthesisToken)) {
            return parse_expression(&expression_tokens[1..close_paren_index]);
        }
    }

    if let Some(assignment_index) = expression_tokens.iter()
        .position(|token| matches!(token.type_, TokenType::AssignmentOperatorToken)) {
        if matches!(expression_tokens[assignment_index-1].type_, TokenType::NameIdentifierToken) {
            let child_assignment_node = match expression_tokens[assignment_index].value {
                None => parse_expression(&expression_tokens[(assignment_index + 1)..]),
                Some(_) => SyntaxTreeNode {
                    value: Some("=".to_string()),
                    type_: SyntaxTreeNodeType::Expression,
                    children: Vec::from([
                        parse_expression(&expression_tokens[assignment_index-1..assignment_index]),
                        parse_expression(&expression_tokens[assignment_index+1..]),
                    ]),
                }
            };

            return SyntaxTreeNode {
                value: expression_tokens[assignment_index-1].value.clone(),
                type_: SyntaxTreeNodeType::Assignment,
                children: Vec::from([child_assignment_node]),
            }
        }
    }

    if matches!(expression_tokens[0].type_, TokenType::NameIdentifierToken)
        && matches!(expression_tokens[1].type_, TokenType::OpenParenthesisToken)
        && matches!(expression_tokens.last().unwrap().type_, TokenType::CloseParenthesisToken) {
        return SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children: parse_collection(&expression_tokens[2..expression_tokens.len()-1]),
        }
    }


    if matches!(expression_tokens[0].type_, TokenType::OpenCollectionToken)
        && matches!(expression_tokens[expression_tokens.len()-1].type_, TokenType::CloseCollectionToken) {
        return SyntaxTreeNode {
            value: None,
            type_: SyntaxTreeNodeType::Collection,
            children: parse_collection(&expression_tokens[1..expression_tokens.len()-1])
        }
    }

    if let Some(numeric_operator_index) = expression_tokens.iter().position(|token| matches!(token.type_, TokenType::NumericOperationToken)) {
        return SyntaxTreeNode {
            value: expression_tokens[numeric_operator_index].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children: Vec::from([
                parse_expression(&expression_tokens[..numeric_operator_index]),
                parse_expression(&expression_tokens[numeric_operator_index+1..]),
            ]),
        }
    }

    if let Some(equality_index) = expression_tokens.iter().position(|token| matches!(token.type_, TokenType::EqualityOperatorToken)) {
        return SyntaxTreeNode {
            value: None,
            type_: SyntaxTreeNodeType::EqualityCheck,
            children: Vec::from([
                parse_expression(&expression_tokens[..equality_index]),
                parse_expression(&expression_tokens[(equality_index+1)..]),
            ]),
        }
    }

    if matches!(expression_tokens[0].type_, TokenType::NameIdentifierToken)
        && matches!(expression_tokens[1].type_, TokenType::DotMethodToken)
        && matches!(expression_tokens[2].type_, TokenType::NameIdentifierToken) {
        let mut children: Vec<SyntaxTreeNode> = Vec::new();
        children.push(parse_expression(&expression_tokens[0..=0]));
        children.extend(parse_collection(&expression_tokens[4..expression_tokens.len()-1]));

        return SyntaxTreeNode {
            value: expression_tokens[2].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children: children,
        }
    }

    panic!("Failed to parse expressions")
}

fn parse_collection(collection_tokens: &[Token]) -> Vec<SyntaxTreeNode> {
    let mut collection_nodes: Vec<SyntaxTreeNode> = Vec::new();

    let mut i = 0;
    for j in i..collection_tokens.len() + 1 {
        if j < collection_tokens.len() && !matches!(collection_tokens[j].type_, TokenType::CommaToken) {
            continue;
        }

        collection_nodes.push(parse_expression(&collection_tokens[i..j]));
        i = j + 1;
    }

    collection_nodes
}
