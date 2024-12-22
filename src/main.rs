use std::collections::{HashMap};
use std::str::Chars;
use crate::JsonValue::Bool;
use crate::Token::{Boolean, Colon, Comma, CurlyClose, CurlyOpen, Null, SquareClose, SquareOpen};

enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

enum Token {
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
    Colon,
    Comma,
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
}


fn main() {
    let json = "true";

    match tokenize(json) {
        Ok(t) => {},
        Err(e) => println!("{}", e)
    }
}

fn tokenize(json: &str) -> Result<Vec<Token>, String> {
    let mut chars = json.chars();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(CurlyOpen),
            '}' => tokens.push(CurlyClose),
            '[' => tokens.push(SquareOpen),
            ']' => tokens.push(SquareClose),
            ':' => tokens.push(Colon),
            ',' => tokens.push(Comma),
            'n' => {
                parse_null(&mut chars);
                tokens.push(Null)
            },
            't' | 'f' =>  tokens.push(parse_bool(&mut chars, c)?),
            // c if c.is_numeric() => {tokens.push(parse_number(&mut chars))}


            c if c.is_ascii_whitespace() => {}
            _ => return Err(format!("Unknown character: {}", c))
        }
    }
    Ok(tokens)
}

fn parse_null(chars: &mut Chars) -> Result<Token, String> {
    let expected = "null";

    for expected_char in expected.chars().skip(1) {
        match chars.next() {
            Some(c) if c == expected_char => continue,
            Some(c) => return Err(format!("Unexpected character '{}' in null token", c)),
            None => return Err("Unexpected end of input while parsing null".to_string())
        }
    }
    Ok(Null)
}

fn parse_bool(chars: &mut Chars, char: char) -> Result<Token, String> {
    let expected = match char {
        't' => "true",
        'f' => "false",
        _ => return Err(format!("Unknown character: {}", char))
    };

    for expected_char in expected.chars().skip(1) {
        match chars.next() {
            Some(c) if c == expected_char => continue,
            Some(c) => return Err(format!("Unexpected character '{}' in boolean token", c)),
            None => return Err("Unexpected end of input while parsing boolean".to_string())
        }
    }

    let result = match expected {
        "true" => true,
        "false" => false,
        _ => unreachable!()
    };

    Ok(Boolean(result))
}



