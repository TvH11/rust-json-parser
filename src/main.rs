mod tokenizer;
mod parser;

use std::collections::{HashMap};
use std::{env, fs, io};
use std::process::exit;

#[derive(Debug)]
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


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    let file_path = &args[1];

    let content = fs::read_to_string(file_path)?;
    //
    // let json =
    //     "{\n\t\"test\":\"hoi\",\n\t\"testObject\":{\n\t\t\"value\":\"hallo\",\n\t},\n\t\"array\":[true,1,null,false],\n}";

    let json = &content;

    println!("Parsing:\n{}", json);

    match tokenizer::tokenize(json) {
        Ok(t) => {
            println!("\ntokens:\n{:?}", t);
            let object = parser::parse(t);
            match object {
                Ok(v) => println!("\njson object:\n{:?}", v),
                Err(e) => println!("{}", e)
            }

        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}





