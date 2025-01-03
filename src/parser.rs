use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::{JsonValue, Token};
use crate::JsonValue::Object;

const END_OF_INPUT: &str = "Unexpected end of input.";

pub fn parse(tokens: Vec<Token>) -> Result<JsonValue, String> {
    let mut tokens_iter = tokens.iter().peekable();
    parse_value(&mut tokens_iter)
}

fn parse_value(tokens: &mut Peekable<Iter<Token>>) -> Result<JsonValue, String> {
    match tokens.next() {
        Some(Token::CurlyOpen) => parse_object(tokens),
        Some(Token::SquareOpen) => parse_array(tokens),
        Some(Token::String(ref s)) => Ok(JsonValue::String(s.clone())),
        Some(Token::Boolean(ref b)) => Ok(JsonValue::Bool(b.clone())),
        Some(Token::Number(ref n)) => Ok(JsonValue::Number(n.clone())),
        Some(Token::Null) => Ok(JsonValue::Null),
        Some(t) => Err(format!("Unexpected token '{:?}' in parse_value", t)),
        None => Err(END_OF_INPUT.to_string())
    }
}

fn parse_array(tokens: &mut Peekable<Iter<Token>>) -> Result<JsonValue, String> {
    let mut array: Vec<JsonValue> = Vec::new();
    while let Some(&&ref t) = tokens.peek() {
        match t {
            Token::SquareClose => {
                tokens.next();
                break
            }
            _ => {
                let value = parse_value(tokens)?;
                array.push(value);

                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                        continue
                    },
                    Some(Token::SquareClose) => continue,
                    Some(t) => return Err(format!("Expected ',' but got '{:?}'", t)),
                    None => return Err("Unexpected end of input while matching array".to_string())
                }
            }
        }
    }
    Ok(JsonValue::Array(array))
}

fn parse_object(tokens: &mut Peekable<Iter<Token>>) -> Result<JsonValue, String> {
    let mut object: HashMap<String, JsonValue> = HashMap::new();
    while let Some(&&ref t) = tokens.peek() {
        match t {
            Token::CurlyClose => {
                tokens.next();
                break
            },
            Token::String(ref s) => {
                tokens.next();
                match tokens.next() {
                    Some(Token::Colon) => {
                        let value = parse_value(tokens)?;
                        object.insert(s.clone(), value);
                    },
                    Some(t) => return Err(format!("Expected ':' but got '{:?}'", t)),
                    None => return Err("Unexpected end of input while trying to match key-value pair".to_string())
                }

                match tokens.peek() {
                    Some(Token::CurlyClose) => continue,
                    Some(Token::Comma) => {
                        tokens.next();
                        continue
                    },
                    Some(t) => return Err(format!("Expected ',' or '}}' but got '{:?}'", t)),
                    None => return Err("Unexpected end of input while trying to match end of key-value pair".to_string())
                }
            }
            _ => {
                return Err(format!("Unexpected token '{:?}' in parse_object", t))
            }
        }
    }
    Ok(Object(object))
}

