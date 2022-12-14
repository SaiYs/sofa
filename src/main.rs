mod ast;
mod cli;
mod codegen;
mod lexer;
mod parser;
mod ty;

use clap::Parser;
use std::{
    fs::File,
    io::{stdout, Read},
};

fn main() {
    // read option
    let args = cli::SofaC::parse();

    // read input source
    let source = args
        .console
        .or_else(|| {
            let mut f = File::open(args.file.unwrap()).unwrap();
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            Some(buf)
        })
        .unwrap();

    // tokenize source into tokens
    let tokens = lexer::tokenize(&source);

    // parse tokens
    let parser = parser::SofaParser::new(&tokens);
    let ast = parser.parse();

    // generate assembly
    if args.stdout {
        let mut generater = codegen::SofaGenerater::new(stdout());
        generater.gen(&ast);
    } else {
        let out = args.out.unwrap_or_else(|| "tmp.s".to_string());
        let mut generater = codegen::SofaGenerater::new(
            std::fs::File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open(out)
                .unwrap(),
        );
        generater.gen(&ast);
    }
}

#[test]
fn test_example() {
    let s = include_str!("../example/test.sofa");
    let tokens = lexer::tokenize(s);
    // dbg!(&tokens);

    let parser = parser::SofaParser::new(&tokens);
    let ast = parser.parse();
    dbg!(&ast);

    let mut generater = codegen::SofaGenerater::new(std::io::stdout());
    generater.gen(&ast);
}
