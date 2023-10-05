use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqual,
    LesserThanOrEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Null,
    String(String),
    Number(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeLiteral {
    Any,
    String,
    Number,
    Boolean,
}

impl Literal {
    pub fn to_string(self) -> String {
        match self {
            Literal::Null => String::from("null"),
            Literal::Number(number) => format!("{}", number),
            Literal::String(string) => format!("\"{}\"", string),
            Literal::Boolean(boolean) => {
                format!(
                    "{}",
                    match boolean {
                        true => "true",
                        _ => "false",
                    }
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Let,
    If,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    TypeLiteral(TypeLiteral),
    Identifier(String),
    Literal(Literal),
    Operator(Operator),
    Keyword(Keyword),

    Arrow,

    Comma,
    Semi,
    Assign,
    Colon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Newline,
    Fin,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: u16,
    pub index: usize,
    pub symbol: Symbol,
}

fn scan_symbol(char: char) -> Option<Symbol> {
    match char {
        '+' => Option::Some(Symbol::Operator(Operator::Plus)),
        '-' => Option::Some(Symbol::Operator(Operator::Minus)),
        '*' => Option::Some(Symbol::Operator(Operator::Star)),
        '/' => Option::Some(Symbol::Operator(Operator::Slash)),
        '(' => Option::Some(Symbol::LeftParen),
        ')' => Option::Some(Symbol::RightParen),
        ',' => Option::Some(Symbol::Comma),
        ';' => Option::Some(Symbol::Semi),
        '=' => Option::Some(Symbol::Assign),
        '{' => Option::Some(Symbol::LeftBrace),
        '}' => Option::Some(Symbol::RightBrace),
        '>' => Option::Some(Symbol::Operator(Operator::GreaterThan)),
        '<' => Option::Some(Symbol::Operator(Operator::LesserThan)),
        '&' => Option::Some(Symbol::Operator(Operator::And)),
        '|' => Option::Some(Symbol::Operator(Operator::Or)),
        '!' => Option::Some(Symbol::Operator(Operator::Not)),
        ':' => Option::Some(Symbol::Colon),
        // '\n' => Option::Some(Symbol::Newline),
        _ => Option::None,
    }
}

fn scan_string(index: usize, chars: &mut Peekable<Enumerate<Chars>>) -> Result<Token, String> {
    let mut value = String::new();

    loop {
        if let Some((_, char)) = chars.next() {
            if char == '"' {
                break;
            }

            value.push(char);
            continue;
        }

        return Err(format!(
            "Unterminated string (\"{}), at index: {}",
            value, index
        ));
    }

    return Ok(Token {
        line: 0,
        index,
        symbol: Symbol::Literal(Literal::String(value)),
    });
}

fn scan_numeric(
    start_char: char,
    start_index: usize,
    chars: &mut Peekable<Enumerate<Chars>>,
) -> Result<Token, String> {
    let mut source = String::from(start_char);
    let mut has_matched_period = false;

    loop {
        match chars.peek() {
            None => break,
            Some((_, char)) if char.is_whitespace() => break,
            Some((_, char)) if char.is_numeric() => source.push(*char),
            Some((_, char)) if *char == '.' && !has_matched_period => {
                has_matched_period = true;
                source.push(*char);
            }
            Some((_, _)) => break,
        }
        chars.next();
    }

    let value: f32 = source.parse().unwrap();
    return Ok(Token {
        line: 0,
        index: start_index,
        symbol: Symbol::Literal(Literal::Number(value)),
    });
}

fn get_symbol_from_identifier(identifier: String) -> Symbol {
    match identifier.as_str() {
        "let" => Symbol::Keyword(Keyword::Let),
        "if" => Symbol::Keyword(Keyword::If),
        "else" => Symbol::Keyword(Keyword::Else),
        "true" => Symbol::Literal(Literal::Boolean(true)),
        "false" => Symbol::Literal(Literal::Boolean(false)),
        "null" => Symbol::Literal(Literal::Null),
        "any" => Symbol::TypeLiteral(TypeLiteral::Any),
        "number" => Symbol::TypeLiteral(TypeLiteral::Number),
        "string" => Symbol::TypeLiteral(TypeLiteral::String),
        "boolean" => Symbol::TypeLiteral(TypeLiteral::Boolean),
        _ => Symbol::Identifier(identifier),
    }
}

fn scan_indentifier(
    start_char: char,
    start_index: usize,
    chars: &mut Peekable<Enumerate<Chars>>,
) -> Result<Token, String> {
    let mut value = String::from(start_char);

    loop {
        match chars.peek() {
            None => break,
            Some((_, char)) if char.is_whitespace() => break,
            Some((_, char)) if *char == '_' || char.is_alphanumeric() => value.push(*char),
            Some((_, _)) => break,
        };
        chars.next();
    }

    return Ok(Token {
        line: 0,
        index: start_index,
        symbol: get_symbol_from_identifier(value),
    });
}

fn scan_alphanumeric(
    char: char,
    index: usize,
    chars: &mut Peekable<Enumerate<Chars>>,
) -> Result<Token, String> {
    if char.is_numeric() {
        return scan_numeric(char, index, chars);
    }

    return scan_indentifier(char, index, chars);
}

fn scan_special(
    char: char,
    index: usize,
    chars: &mut Peekable<Enumerate<Chars>>,
) -> Result<Token, String> {
    match char {
        '\"' => scan_string(index, chars),
        '!' if chars.peek().is_some_and(|(_, ch)| *ch == '=') => {
            if let Some((_, '=')) = chars.next() {
                Ok(Token {
                    line: 0,
                    index: index,
                    symbol: Symbol::Operator(Operator::NotEqual),
                })
            } else {
                Err(String::from("! was not followed by ="))
            }
        }
        '<' | '>' if chars.peek().is_some_and(|(_, ch)| *ch == '=') => {
            chars.next();
            Ok(Token {
                line: 0,
                index,
                symbol: match char {
                    '>' => Symbol::Operator(Operator::GreaterThanOrEqual),
                    '<' => Symbol::Operator(Operator::LesserThanOrEqual),
                    _ => return Err(String::from("Help!")),
                },
            })
        }
        '=' if chars.peek().is_some_and(|(_, ch)| *ch == '>' || *ch == '=') => match chars.next() {
            Some((_, '>')) => Ok(Token {
                line: 0,
                index,
                symbol: Symbol::Arrow,
            }),
            Some((_, '=')) => Ok(Token {
                line: 0,
                index,
                symbol: Symbol::Operator(Operator::Equal),
            }),
            _ => Err(String::from("gg")),
        },
        _ => Err("scan special was given a char that it cannot process".to_owned()),
    }
}

fn get_next_token(chars: &mut Peekable<Enumerate<Chars>>) -> Result<Token, String> {
    match chars.next() {
        Some((index, char)) => match char {
            '\"' => scan_special(char, index, chars),
            '!' if chars.peek().is_some_and(|(_, ch)| *ch == '=') => {
                scan_special(char, index, chars)
            }
            '<' | '>' if chars.peek().is_some_and(|(_, ch)| *ch == '=') => {
                scan_special(char, index, chars)
            }
            '=' if chars.peek().is_some_and(|(_, ch)| *ch == '>' || *ch == '=') => {
                scan_special(char, index, chars)
            }
            _ if char.is_whitespace() => get_next_token(chars),
            _ if char.is_alphanumeric() => scan_alphanumeric(char, index, chars),
            _ => match scan_symbol(char) {
                Some(symbol) => Ok(Token {
                    line: 0,
                    index,
                    symbol,
                }),
                None => Err(format!(
                    "Unrecognized symbol: \"{}\", at index: {}",
                    char, index
                )),
            },
        },
        None => Ok(Token {
            line: 0,
            index: 0,
            symbol: Symbol::Fin,
        }),
    }
}

pub fn scan(source: &String) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().enumerate().peekable();
    let mut tokens = Vec::new();

    loop {
        let token = get_next_token(&mut chars)?;
        tokens.push(token.clone());

        if token.symbol == Symbol::Fin {
            break;
        }
    }

    return Result::Ok(tokens);
}
