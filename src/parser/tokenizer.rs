use std::str;
use std::vec;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    CurlyBracket,       // '{', '}'
    SquareBracket,      // '{', '}'
    Colon,              // ':'
    Comma,              // ','
    True,               // 'true'
    False,              // 'false'
    Null,               // 'null'
    String,             // '"any string"'
    Number,             // '86', '-123'
    Unknown(Vec<char>), // Unknown
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Vec<char>,
}

fn get_symbol_token_kind(symbol: char) -> Option<TokenKind> {
    use TokenKind::*;
    match symbol {
        '{' | '}' => Some(CurlyBracket),
        '[' | ']' => Some(SquareBracket),
        ':' => Some(Colon),
        ',' => Some(Comma),
        _ => None,
    }
}

fn is_whitespace(symbol: char) -> bool {
    symbol == ' ' || symbol == '\n'
}

fn is_number(symbol: char) -> bool {
    symbol >= '0' && symbol <= '9'
}

fn try_parse_literal_token(literal: &str, current: usize, chars: &Vec<char>) -> Option<Token> {
    let prefix = literal.chars().nth(0);
    if prefix.is_none() {
        return None;
    }

    let prefix = prefix.unwrap();
    if prefix != chars[current] {
        return None;
    }

    if chars.len() <= current + literal.len() {
        return None;
    }

    let next_chars: String = chars[current..current + literal.len()]
        .iter()
        .cloned()
        .collect();
    println!("next_chars: {} literal: {}", next_chars, literal);
    if next_chars != literal {
        return None;
    }

    let token_value: Vec<char> = next_chars.chars().into_iter().collect();
    let token_kind = match literal {
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "null" => TokenKind::Null,
        _ => TokenKind::Unknown(token_value.clone()),
    };

    Some(Token {
        kind: token_kind,
        value: token_value,
    })
}

pub fn tokenize(chars: &Vec<char>) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut current = 0;

    'outer: while current < chars.len() {
        let mut char = chars[current];

        if is_whitespace(char) {
            current += 1;
            continue;
        }

        // process '{', '}', '[', ']', ':', ','
        if let Some(token_kind) = get_symbol_token_kind(char) {
            tokens.push(Token {
                kind: token_kind,
                value: vec![char],
            });
            current += 1;
            continue;
        }

        // process string
        if char == '"' {
            let mut string_chars = vec![];

            current += 1;
            char = chars[current];
            let mut escape = false;

            while char != '"' || (char == '"' && escape) {
                if escape {
                    escape = !escape
                };
                if char == '\\' {
                    escape = true
                };
                string_chars.push(char);

                current += 1;
                char = chars[current];
            }

            tokens.push(Token {
                kind: TokenKind::String,
                value: string_chars,
            });

            // skip '"'
            current += 1;
            continue;
        }

        // process number
        if is_number(char) || char == '-' {
            let mut number_chars = vec![char];

            current += 1;
            char = chars[current];
            while is_number(char) {
                number_chars.push(char);
                current += 1;
                char = chars[current];
            }

            tokens.push(Token {
                kind: TokenKind::Number,
                value: number_chars,
            });
            continue;
        }

        // process true, false, null
        for &literal in ["true", "false", "null"].iter() {
            if let Some(token) = try_parse_literal_token(literal, current, chars) {
                tokens.push(token);
                current += literal.len();

                continue 'outer; // care this should continue outer loop
            }
        }

        // no match case
        break;
    }

    return Ok(tokens);
}
