use crate::syntax_tree::syntax_tree_node::{SyntaxTreeNode, SyntaxTreeNodeType};
use crate::tokenizer::token::{Token, TokenType};

pub(crate) fn parse(source_tokens: Vec<Token>) -> Result<SyntaxTreeNode, String> {
    if !matches!(source_tokens[0].type_, TokenType::NamespaceToken)
        || !matches!(source_tokens[1].type_, TokenType::NameIdentifierToken)
    {
        return Err(String::from("no token namespace found"));
    }

    let mut current_index: usize = 0;
    let mut internal_nodes: Vec<SyntaxTreeNode> = vec![];

    while current_index < source_tokens.len() {
        match (
            source_tokens.get(current_index),
            source_tokens.get(current_index + 1),
            source_tokens.get(current_index + 2),
        ) {
            (Some(first), Some(second), Some(third))
                if matches!(first.type_, TokenType::TypeDeclarationToken)
                    && matches!(second.type_, TokenType::NameIdentifierToken)
                    && matches!(third.type_, TokenType::OpenParenthesisToken) =>
            {
                let end_index: usize =
                    match find_index_of_last_closing_scope(&source_tokens, current_index) {
                        Some(index) => index,
                        None => return Err("cannot find end of scope".to_string()),
                    };

                match parse_method(&source_tokens[current_index..=end_index]) {
                    Ok(valid_node) => {
                        internal_nodes.push(valid_node);
                        current_index = end_index;
                    }
                    Err(error) => return Err(error),
                }
            }
            (Some(first), Some(second), _)
                if matches!(first.type_, TokenType::ClassToken)
                    && matches!(second.type_, TokenType::NameIdentifierToken) =>
            {
                todo!()
            }
            (Some(first), Some(second), _)
                if matches!(first.type_, TokenType::NameIdentifierToken)
                    && matches!(second.type_, TokenType::OpenParenthesisToken) =>
            {
                let semicolon_index: usize = match source_tokens
                    .iter()
                    .position(|token| matches!(token.type_, TokenType::SemicolonToken))
                {
                    Some(index) => current_index + index,
                    None => source_tokens.len(),
                };

                match parse_expression(&source_tokens[current_index..semicolon_index]) {
                    Ok(valid_node) => {
                        internal_nodes.push(valid_node);
                        current_index = semicolon_index;
                    }
                    Err(error) => return Err(error),
                }
            }
            _ => current_index += 1,
        }
    }

    Ok(SyntaxTreeNode {
        value: source_tokens[1].value.clone(),
        type_: SyntaxTreeNodeType::Namespace,
        children: internal_nodes,
    })
}

fn find_index_of_last_closing_scope(
    source_tokens: &[Token],
    index_to_start: usize,
) -> Option<usize> {
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

fn parse_method(method_tokens: &[Token]) -> Result<SyntaxTreeNode, String> {
    let argument_open_parenthesis_index: usize = method_tokens
        .iter()
        .position(|token| matches!(token.type_, TokenType::OpenParenthesisToken))
        .unwrap();
    let argument_close_parenthesis_index: usize = method_tokens
        .iter()
        .position(|token| matches!(token.type_, TokenType::CloseParenthesisToken))
        .unwrap();
    let method_argument_tokens: &[Token] =
        &method_tokens[(argument_open_parenthesis_index + 1)..argument_close_parenthesis_index];

    let method_body_tokens: &[Token] = &method_tokens[argument_close_parenthesis_index + 2..];

    let mut method_nodes: Vec<SyntaxTreeNode> = vec![];
    method_nodes.extend(match parse_method_arguments(method_argument_tokens) {
        Ok(valid_node) => valid_node,
        Err(error) => return Err(error),
    });
    method_nodes.extend(match parse_internal_scope(method_body_tokens) {
        Ok(valid_node) => valid_node,
        Err(error) => return Err(error),
    });

    Ok(SyntaxTreeNode {
        value: method_tokens[1].value.clone(),
        type_: SyntaxTreeNodeType::Method,
        children: method_nodes,
    })
}

fn parse_method_arguments(argument_tokens: &[Token]) -> Result<Vec<SyntaxTreeNode>, String> {
    Ok(argument_tokens
        .iter()
        .filter(|token| matches!(token.type_, TokenType::NameIdentifierToken))
        .map(|token| SyntaxTreeNode {
            value: token.value.clone(),
            type_: SyntaxTreeNodeType::MethodArgument,
            children: vec![],
        })
        .collect::<Vec<SyntaxTreeNode>>())
}

fn parse_internal_scope(internal_tokens: &[Token]) -> Result<Vec<SyntaxTreeNode>, String> {
    let mut scope_nodes: Vec<SyntaxTreeNode> = vec![];

    let mut token_index: usize = 0;
    while token_index < internal_tokens.len() {
        let token: &Token = &internal_tokens[token_index];

        match token.type_ {
            TokenType::ReturnToken | TokenType::CloseScopeToken => {
                token_index += 1;
            }
            TokenType::CommentToken => {
                scope_nodes.push(SyntaxTreeNode {
                    value: token.value.clone(),
                    type_: SyntaxTreeNodeType::Comment,
                    children: vec![],
                });
                token_index += 1;
            }
            _ => {
                let mut end_of_scope_index: usize = token_index
                    + internal_tokens
                        .iter()
                        .skip(token_index)
                        .position(|token| matches!(token.type_, TokenType::SemicolonToken))
                        .unwrap_or(0);

                if internal_tokens
                    .iter()
                    .skip(token_index)
                    .position(|token| matches!(token.type_, TokenType::OpenScopeToken))
                    .is_some_and(|open_scope_index| {
                        (token_index + open_scope_index) < end_of_scope_index
                    })
                {
                    end_of_scope_index =
                        find_index_of_last_closing_scope(&internal_tokens, token_index).unwrap();
                }

                scope_nodes.push(
                    match parse_expression(&internal_tokens[token_index..=end_of_scope_index]) {
                        Ok(valid_node) => valid_node,
                        Err(error) => return Err(error),
                    },
                );

                token_index = end_of_scope_index + 1;
            }
        }
    }

    Ok(group_consecutive_assignments(scope_nodes))

    // scope_nodes
}

fn group_consecutive_assignments(body_nodes: Vec<SyntaxTreeNode>) -> Vec<SyntaxTreeNode> {
    let mut body_nodes_with_compound_assignments: Vec<SyntaxTreeNode> = vec![];

    let mut i: usize = 0;
    while i < body_nodes.len() {
        if !matches!(body_nodes[i].type_, SyntaxTreeNodeType::Assignment) {
            body_nodes_with_compound_assignments.push(body_nodes[i].clone());
            i += 1;
            continue;
        }

        let mut j: usize = i + 1;
        while j < body_nodes.len() && matches!(body_nodes[j].type_, SyntaxTreeNodeType::Assignment)
        {
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

fn parse_expression(expression_tokens: &[Token]) -> Result<SyntaxTreeNode, String> {
    let expression_tokens: &[Token] = match expression_tokens.last().unwrap().type_ {
        TokenType::SemicolonToken => &expression_tokens[..(expression_tokens.len() - 1)],
        _ => expression_tokens,
    };

    if expression_tokens.len() == 1 {
        return Ok(SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Literal,
            children: vec![],
        });
    }

    if matches!(expression_tokens[0].type_, TokenType::ReturnToken) {
        return parse_expression(&expression_tokens[1..]);
    }

    if matches!(expression_tokens[0].type_, TokenType::OpenParenthesisToken) {
        if let Some(close_paren_index) = expression_tokens
            .iter()
            .rposition(|token| matches!(token.type_, TokenType::CloseParenthesisToken))
        {
            return parse_expression(&expression_tokens[1..close_paren_index]);
        }
    }

    if matches!(&expression_tokens[0], Token { type_: TokenType::BranchingOperatorToken, value: Some(i)} if i == "if")
    {
        let close_paren_index: usize = expression_tokens
            .iter()
            .position(|token| matches!(token.type_, TokenType::CloseParenthesisToken))
            .unwrap();

        let mut children: Vec<SyntaxTreeNode> = Vec::from([
            match parse_expression(&expression_tokens[2..close_paren_index]) {
                Ok(valid_node) => valid_node,
                Err(error) => return Err(error),
            },
        ]);
        children.extend(
            match parse_internal_scope(&expression_tokens[close_paren_index + 2..]) {
                Ok(valid_nodes) => valid_nodes,
                Err(error) => return Err(error),
            },
        );

        return Ok(SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Branch,
            children,
        });
    }

    if matches!(&expression_tokens[0], Token { type_: TokenType::BranchingOperatorToken, value: Some(i)} if i == "else")
    {
        return Ok(SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Branch,
            children: match parse_internal_scope(&expression_tokens[2..]) {
                Ok(valid_nodes) => valid_nodes,
                Err(error) => return Err(error),
            },
        });
    }

    if let Some(assignment_index) = expression_tokens
        .iter()
        .position(|token| matches!(token.type_, TokenType::AssignmentOperatorToken))
    {
        if matches!(
            expression_tokens[assignment_index - 1].type_,
            TokenType::NameIdentifierToken
        ) {
            let child_assignment_node = match expression_tokens[assignment_index].value {
                None => match parse_expression(&expression_tokens[(assignment_index + 1)..]) {
                    Ok(valid_node) => valid_node,
                    Err(error) => return Err(error),
                },
                Some(_) => SyntaxTreeNode {
                    value: Some("=".to_string()),
                    type_: SyntaxTreeNodeType::Expression,
                    children: Vec::from([
                        match parse_expression(
                            &expression_tokens[assignment_index - 1..assignment_index],
                        ) {
                            Ok(valid_node) => valid_node,
                            Err(error) => return Err(error),
                        },
                        match parse_expression(&expression_tokens[assignment_index + 1..]) {
                            Ok(valid_node) => valid_node,
                            Err(error) => return Err(error),
                        },
                    ]),
                },
            };

            return Ok(SyntaxTreeNode {
                value: expression_tokens[assignment_index - 1].value.clone(),
                type_: SyntaxTreeNodeType::Assignment,
                children: Vec::from([child_assignment_node]),
            });
        }
    }

    if matches!(expression_tokens[0].type_, TokenType::NameIdentifierToken)
        && matches!(expression_tokens[1].type_, TokenType::OpenParenthesisToken)
        && matches!(
            expression_tokens.last().unwrap().type_,
            TokenType::CloseParenthesisToken
        )
    {
        return Ok(SyntaxTreeNode {
            value: expression_tokens[0].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children: match parse_collection(&expression_tokens[2..expression_tokens.len() - 1]) {
                Ok(valid_node) => valid_node,
                Err(error) => return Err(error),
            },
        });
    }

    if matches!(expression_tokens[0].type_, TokenType::OpenCollectionToken)
        && matches!(
            expression_tokens[expression_tokens.len() - 1].type_,
            TokenType::CloseCollectionToken
        )
    {
        return Ok(SyntaxTreeNode {
            value: None,
            type_: SyntaxTreeNodeType::Collection,
            children: match parse_collection(&expression_tokens[1..expression_tokens.len() - 1]) {
                Ok(valid_node) => valid_node,
                Err(error) => return Err(error),
            },
        });
    }

    if let Some(numeric_operator_index) = expression_tokens
        .iter()
        .position(|token| matches!(token.type_, TokenType::NumericOperationToken))
    {
        return Ok(SyntaxTreeNode {
            value: expression_tokens[numeric_operator_index].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children: Vec::from([
                match parse_expression(&expression_tokens[..numeric_operator_index]) {
                    Ok(valid_node) => valid_node,
                    Err(error) => return Err(error),
                },
                match parse_expression(&expression_tokens[numeric_operator_index + 1..]) {
                    Ok(valid_node) => valid_node,
                    Err(error) => return Err(error),
                },
            ]),
        });
    }

    if let Some(equality_index) = expression_tokens
        .iter()
        .position(|token| matches!(token.type_, TokenType::EqualityOperatorToken))
    {
        return Ok(SyntaxTreeNode {
            value: Some("=".to_string()),
            type_: SyntaxTreeNodeType::EqualityCheck,
            children: Vec::from([
                match parse_expression(&expression_tokens[..equality_index]) {
                    Ok(valid_node) => valid_node,
                    Err(error) => return Err(error),
                },
                match parse_expression(&expression_tokens[(equality_index + 1)..]) {
                    Ok(valid_node) => valid_node,
                    Err(error) => return Err(error),
                },
            ]),
        });
    }

    if matches!(expression_tokens[0].type_, TokenType::NameIdentifierToken)
        && matches!(expression_tokens[1].type_, TokenType::DotMethodToken)
        && matches!(expression_tokens[2].type_, TokenType::NameIdentifierToken)
    {
        let mut children: Vec<SyntaxTreeNode> = vec![];
        children.push(match parse_expression(&expression_tokens[0..=0]) {
            Ok(valid_node) => valid_node,
            Err(error) => return Err(error),
        });
        children.extend(
            match parse_collection(&expression_tokens[4..expression_tokens.len() - 1]) {
                Ok(valid_node) => valid_node,
                Err(error) => return Err(error),
            },
        );

        return Ok(SyntaxTreeNode {
            value: expression_tokens[2].value.clone(),
            type_: SyntaxTreeNodeType::Expression,
            children,
        });
    }

    Err(format!(
        "Failed to parse expressions: {}",
        expression_tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

fn parse_collection(collection_tokens: &[Token]) -> Result<Vec<SyntaxTreeNode>, String> {
    let mut collection_nodes: Vec<SyntaxTreeNode> = vec![];

    let mut i: usize = 0;
    for j in i..collection_tokens.len() + 1 {
        if j < collection_tokens.len()
            && !matches!(collection_tokens[j].type_, TokenType::CommaToken)
        {
            continue;
        }

        collection_nodes.push(match parse_expression(&collection_tokens[i..j]) {
            Ok(valid_node) => valid_node,
            Err(error) => return Err(error),
        });
        i = j + 1;
    }

    Ok(collection_nodes)
}
