use std::iter::Peekable;
use std::str::Chars;
use crate::Token;
use crate::Token::{Boolean, Colon, Comma, CurlyClose, CurlyOpen, Null, Number, SquareClose, SquareOpen};

pub fn tokenize(json: &str) -> Result<Vec<Token>, String> {
    let mut chars = json.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(CurlyOpen),
            '}' => tokens.push(CurlyClose),
            '[' => tokens.push(SquareOpen),
            ']' => tokens.push(SquareClose),
            ':' => tokens.push(Colon),
            ',' => tokens.push(Comma),
            'n' => tokens.push(parse_null(&mut chars)?),
            't' | 'f' =>  tokens.push(parse_bool(&mut chars, c)?),
            '"' => tokens.push(parse_string(&mut chars)),
            c if c.is_numeric() => tokens.push(parse_number(&mut chars, c)?),


            c if c.is_ascii_whitespace() => {}
            _ => return Err(format!("Unknown character: {}", c))
        }
    }
    Ok(tokens)
}

fn parse_word(expected: &str, chars: &mut Peekable<Chars>) -> Result<(), String> {
    for expected_char in expected.chars().skip(1) {
        match chars.next() {
            Some(c) if c == expected_char => continue,
            Some(c) => return Err(format!("Unexpected character '{}' in '{}' token", c, expected)),
            None => return Err(format!("Unexpected end of input while parsing token '{}'", expected))
        }
    }
    Ok(())
}

fn parse_null(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let expected = "null";
    match parse_word(expected, chars) {
        Ok(()) => Ok(Null),
        Err(value) => Err(value)
    }
}

fn parse_bool(chars: &mut Peekable<Chars>, char: char) -> Result<Token, String> {
    let expected = match char {
        't' => ("true", true),
        'f' => ("false", false),
        _ => unreachable!()
    };

    match parse_word(expected.0, chars) {
        Ok(()) => Ok(Boolean(expected.1)),
        Err(value) => Err(value)
    }
}

fn parse_number(chars: &mut Peekable<Chars>, c: char) -> Result<Token, String> {
    let mut number = c.to_string();

    while let Some(&c) = chars.peek() {
        if !c.is_numeric() {
            break
        }
        number.push(c);
        chars.next();
    }

    let result = match number.parse::<f64>() {
        Ok(value) => value,
        Err(e) => return Err(format!("Failed to convert '{}' to f64", {number}))
    };

    Ok(Number(result))
}

fn parse_string(chars: &mut Peekable<Chars>) -> Token {
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            _ => string.push(c)
        }
    }
    Token::String(string)
}