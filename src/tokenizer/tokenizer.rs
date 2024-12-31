use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenType;
use std::iter::Peekable;
use std::slice::Iter;

pub fn tokenize(source_code_text: String) -> Result<Vec<Token>, String> {
    let mut token_queue: Vec<Token> = Vec::new();
    let source_queue: Vec<char> = source_code_text.chars().collect::<Vec<char>>();
    let mut source_queue: Peekable<Iter<char>> = source_queue.iter().peekable();

    while let Some(character) = source_queue.next() {
        if char::is_whitespace(*character) {
            continue;
        }

        if *character == '/' && source_queue.peek().is_some_and(|c: &&char| **c == '/') {
            let (parsed_comment, parsed_length): (String, usize) =
                parse_with_predicate(*character, source_queue.clone(), |c| {
                    **c != '\r' && **c != '\n'
                });

            token_queue.push(Token {
                type_: TokenType::CommentToken,
                value: Some(parsed_comment[2..].to_string()),
            });

            for _ in 1..parsed_length {
                source_queue.next();
            }

            continue;
        }

        if char::is_alphanumeric(*character) {
            let (parsed_identifier, parsed_ident_length): (String, usize) = parse_with_predicate(
                *character,
                source_queue.clone(),
                match char::is_alphanumeric(*character) {
                    true => |c: &&char| char::is_alphanumeric(**c) || **c == '<' || **c == '>',
                    false => {
                        |c: &&char| char::is_numeric(**c) || **c == '.' || **c == 'f' || **c == 'd'
                    }
                },
            );

            token_queue.push(match parsed_identifier.as_str() {
                "namespace" => Token {
                    type_: TokenType::NamespaceToken,
                    value: None,
                },
                "class" => Token {
                    type_: TokenType::ClassToken,
                    value: None,
                },
                "var" | "int" | "double" | "string" | "bool" => Token {
                    type_: TokenType::TypeDeclarationToken,
                    value: Some(parsed_identifier),
                },
                generic if is_generic_type(generic) => Token {
                    type_: TokenType::TypeDeclarationToken,
                    value: Some(parsed_identifier),
                },
                "true" | "false" => Token {
                    type_: TokenType::BooleanLiteralToken,
                    value: Some(parsed_identifier),
                },
                "null" => Token {
                    type_: TokenType::NullLiteralToken,
                    value: Some(parsed_identifier),
                },
                "return" => Token {
                    type_: TokenType::ReturnToken,
                    value: Some(parsed_identifier),
                },
                "if" | "else" => Token {
                    type_: TokenType::BranchingOperatorToken,
                    value: Some(parsed_identifier),
                },
                _ if char::is_numeric(*character) => Token {
                    type_: TokenType::NumericLiteralToken,
                    value: Some(parsed_identifier),
                },
                _ => Token {
                    type_: TokenType::NameIdentifierToken,
                    value: Some(parsed_identifier),
                },
            });

            for _ in 1..parsed_ident_length {
                source_queue.next();
            }
        }

        if char::is_ascii_punctuation(character) {
            let (parsed_symbols, parsed_symbol_length): (String, usize) =
                parse_with_predicate(*character, source_queue.clone(), |c| {
                    char::is_ascii_punctuation(*c)
                });

            let matching_token: Option<Token> = match parsed_symbols.as_str() {
                "==" => Some(Token {
                    type_: TokenType::EqualityOperatorToken,
                    value: None,
                }),
                "&&" | "||" => Some(Token {
                    type_: TokenType::BooleanOperationToken,
                    value: Some(parsed_symbols),
                }),
                "+=" | "-=" | "*=" | "/=" => Some(Token {
                    type_: TokenType::AssignmentOperatorToken,
                    value: Some(parsed_symbols),
                }),
                _ => None,
            };

            if matching_token.is_some() {
                for _ in 1..parsed_symbol_length {
                    source_queue.next();
                }
                token_queue.push(matching_token.unwrap());
                continue;
            }

            token_queue.push(match *character {
                '=' => Token {
                    type_: TokenType::AssignmentOperatorToken,
                    value: None,
                },
                '(' => Token {
                    type_: TokenType::OpenParenthesisToken,
                    value: None,
                },
                ')' => Token {
                    type_: TokenType::CloseParenthesisToken,
                    value: None,
                },
                '{' => Token {
                    type_: TokenType::OpenScopeToken,
                    value: None,
                },
                '}' => Token {
                    type_: TokenType::CloseScopeToken,
                    value: None,
                },
                '[' => Token {
                    type_: TokenType::OpenCollectionToken,
                    value: None,
                },
                ']' => Token {
                    type_: TokenType::CloseCollectionToken,
                    value: None,
                },
                ';' => Token {
                    type_: TokenType::SemicolonToken,
                    value: None,
                },
                '+' | '-' | '*' | '/' => Token {
                    type_: TokenType::NumericOperationToken,
                    value: Some(character.to_string()),
                },
                '|' | '&' => Token {
                    type_: TokenType::BooleanOperationToken,
                    value: Some(character.to_string()),
                },
                ',' => Token {
                    type_: TokenType::CommaToken,
                    value: None,
                },
                '.' => Token {
                    type_: TokenType::DotMethodToken,
                    value: None,
                },
                _ => return Err(format!("Unrecognized character '{}'.", *character)),
            });
        }
    }

    Ok(token_queue)
}

fn is_generic_type(text_value: &str) -> bool {
    let generic_type_open_bracket: Option<usize> = text_value.find('<');
    let generic_type_close_bracket: Option<usize> = text_value.find('>');

    match (generic_type_open_bracket, generic_type_close_bracket) {
        (Some(open), Some(close)) => open + 1 != close,
        _ => false,
    }
}

fn parse_with_predicate(
    initial_character: char,
    mut source_queue: Peekable<Iter<char>>,
    predicate: impl Fn(&&char) -> bool,
) -> (String, usize) {
    let mut parsed_characters: String = String::from(initial_character);

    while source_queue.peek().is_some_and(&predicate) {
        parsed_characters.push(*source_queue.next().unwrap());
    }
    let parsed_string_length = parsed_characters.len();

    (parsed_characters, parsed_string_length)
}
