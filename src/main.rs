mod tokenizer;

use std::collections::{HashMap};

enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

#[derive(Debug)]
pub enum Token {
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
    let json = "\"hoi\"truefalsenull1111";

    match tokenizer::tokenize(json) {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{}", e)
    }
}





