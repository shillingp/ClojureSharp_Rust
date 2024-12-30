use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenType;
use std::iter::Peekable;
use std::slice::Iter;

pub fn tokenize(source_code_text: String) -> Vec<Token> {
    let mut token_queue: Vec<Token> = Vec::new();
    let source_queue: Vec<char> = source_code_text.chars().collect::<Vec<char>>();
    let mut source_queue: Peekable<Iter<char>> = source_queue.iter().peekable();

    while let Some(character) = source_queue.next() {
        if char::is_whitespace(*character) {
            continue;
        }

        if *character == '/'
        {
            let mut comment_iterable = source_queue.clone();
            let mut parsed_comment: String = String::from(*character);
            if comment_iterable.peek().is_some_and(|c| **c == '/') {
                while comment_iterable.peek().is_some_and(|c| **c != '\r' && **c != '\n') {
                    parsed_comment.push(*comment_iterable.next().unwrap());
                }
            }
            let parsed_length = parsed_comment.len();

            token_queue.push(Token {
                type_: TokenType::CommentToken,
                value: Some(parsed_comment[2..].to_string())
            });

            for _ in 1..parsed_length {
                source_queue.next();
            }

            continue;
        }

        //if (_sourceCode.Count > 1
        //     && _sourceCode.Peek() is '/'
        //     && _sourceCode.ElementAt(1) is '/')
        // {
        //     StringBuilder commentStringBuilder = new StringBuilder();
        //     while (_sourceCode.Peek() is not ('\r' or '\n'))
        //         commentStringBuilder.Append(_sourceCode.Dequeue());
        //
        //     tokenBuffer.Enqueue(new Token(TokenType.CommentToken, commentStringBuilder.ToString()[2..]));
        //     continue;
        // }

        if char::is_alphanumeric(*character) {
            let mut parsed_identifier: String = String::from(*character);
            if char::is_alphabetic(*character) {
                parsed_identifier.push_str(parse_identifier(&mut source_queue).as_str());
            } else {
                parsed_identifier.push_str(parse_numeric(&mut source_queue).as_str());
            };

            token_queue.push(match parsed_identifier.as_str() {
                "namespace"
                    => Token { type_: TokenType::NamespaceToken, value: None },
                "class"
                    => Token { type_: TokenType::ClassToken, value: None },
                "var" | "int" | "double" | "string" | "bool"
                    => Token { type_: TokenType::TypeDeclarationToken, value: Some(parsed_identifier)},
                generic if is_generic_type(generic)
                    => Token { type_: TokenType::TypeDeclarationToken, value: Some(parsed_identifier)},
                "true" | "false"
                    => Token { type_: TokenType::BooleanLiteralToken, value: Some(parsed_identifier)},
                "null"
                    => Token { type_: TokenType::NullLiteralToken, value: Some(parsed_identifier)},
                "return"
                    => Token { type_: TokenType::ReturnToken, value: Some(parsed_identifier)},
                "if" | "else"
                    => Token { type_: TokenType::BranchingOperatorToken, value: Some(parsed_identifier)},
                _ if char::is_numeric(*character)
                    => Token{ type_: TokenType::NumericLiteralToken, value: Some(parsed_identifier)},
                _
                    => Token { type_: TokenType::NameIdentifierToken, value: Some(parsed_identifier) }
            })
        }

        if char::is_ascii_punctuation(character) {
            let mut symbol_iterable = source_queue.clone();
            let mut parsed_identifier: String = String::from(*character);
            while symbol_iterable.peek().is_some_and(|c| char::is_ascii_punctuation(*c)) {
                parsed_identifier.push(*symbol_iterable.next().unwrap());
            }
            let parsed_length = parsed_identifier.len();

            let matching_token: Option<Token> = match parsed_identifier.as_str() {
                "=="
                    => Some(Token { type_: TokenType::EqualityOperatorToken, value: None }),
                "&&" | "||"
                    => Some(Token { type_: TokenType::BooleanOperationToken, value: Some(parsed_identifier)}),
                "+=" | "-=" | "*=" | "/="
                    => Some(Token { type_: TokenType::AssignmentOperatorToken, value: Some(parsed_identifier)}),
                _ => None,
            };

            if matching_token.is_some(){
                token_queue.push(matching_token.unwrap());
                for _ in 1..parsed_length {
                    source_queue.next();
                }
                continue;
            }

            token_queue.push(match *character {
                '=' => Token { type_: TokenType::AssignmentOperatorToken, value: None },
                '(' => Token { type_: TokenType::OpenParenthesisToken, value: None },
                ')' => Token { type_: TokenType::CloseParenthesisToken, value: None },
                '{' => Token { type_: TokenType::OpenScopeToken, value: None },
                '}' => Token { type_: TokenType::CloseScopeToken, value: None },
                '[' => Token { type_: TokenType::OpenCollectionToken, value: None },
                ']' => Token { type_: TokenType::CloseCollectionToken, value: None },
                ';' => Token { type_: TokenType::SemicolonToken, value: None },
                '+' | '-' | '*' | '/'
                    => Token { type_: TokenType::NumericOperationToken, value: Some(character.to_string()) },
                '|' | '&' => Token { type_: TokenType::BooleanOperationToken, value: Some(character.to_string()) },
                ',' => Token { type_: TokenType::CommaToken, value: None },
                '.' => Token { type_: TokenType::DotMethodToken, value: None },
                _ => panic!("Unrecognized character '{}'.", *character)
            });
        }
    }

    token_queue
}

fn is_generic_type(text_value: &str) -> bool {
    let generic_type_open_bracket: Option<usize> = text_value.find('<');
    let generic_type_close_bracket: Option<usize> = text_value.find('>');

    match (generic_type_open_bracket, generic_type_close_bracket) {
        (Some(open), Some(close)) => open + 1 != close,
        _ => false
    }
}

fn parse_identifier(source_queue: &mut Peekable<Iter<char>>) -> String {
    let mut parsed_characters: String = String::new();

    while source_queue.peek().is_some_and(|c| char::is_alphanumeric(**c) || **c == '<' || **c == '>') {
        parsed_characters.push(*source_queue.next().unwrap());
    }

    parsed_characters
}

fn parse_numeric(source_queue: &mut Peekable<Iter<char>>) -> String {
    let mut parsed_characters: String = String::new();

    while source_queue.peek().is_some_and(|c| char::is_numeric(**c) || **c == '.' || **c == 'f' || **c == 'd') {
        parsed_characters.push(*source_queue.next().unwrap());
    }

    parsed_characters
}

