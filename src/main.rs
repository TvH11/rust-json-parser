mod tokenizer;
mod parser;

use std::collections::{HashMap};
use std::{env, fmt, fs, io};
use std::fmt::Formatter;
use std::process::exit;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

impl JsonValue {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = "\t".repeat(indent);
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(a) => {
                write!(f, "[")?;
                for v in a.iter() {
                    write!(f, "\n\t{}", indent_str)?;
                    v.fmt_with_indent(f, indent + 1)?;
                    write!(f, ",")?;
                }
                write!(f, "\n{}]", indent_str)
            },
            JsonValue::Object(o) => {
                write!(f, "{{")?;
                for (k, v) in o.iter() {
                    write!(f, "\n\t{}{}: ", indent_str, k)?;
                    v.fmt_with_indent(f, indent + 1)?;
                    write!(f, ",")?;
                }
                write!(f, "\n{}}}", indent_str)
            }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
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
                Ok(v) => println!("\njson object:\n{}", v),
                Err(e) => println!("{}", e)
            }

        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}





