use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::{fs::File, vec};

mod parser;
use parser::{tokenize, TokenParser};



fn main() -> std::io::Result<()> {
    let file = File::open("simple.json")?;
    let mut reader = BufReader::new(file);

    let mut buffer = vec![];
    reader.read_to_end(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(s) => println!("input:\n{}\n", s),
        Err(err) => panic!(err),
    };

    let chars: Vec<char> = buffer.iter().map(|b| *b as char).collect();
    let tokens = tokenize(&chars).unwrap();
    println!("tokens:");
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let mut token_parser =TokenParser::new();
    let ast = match token_parser.parse(tokens) {
        Ok(ast) => ast,
        Err(err) => {
            panic!(err)
        },
    };

    println!("ast:\n{:?}", ast);


    Ok(())
}
