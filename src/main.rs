mod codegen;
mod instruction;
mod lexer;
mod parser;
mod symbol;
mod token;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use clap::{App, Arg};

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    // Create the argument parser
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("MICRO-1 machine language assembler written in Rust")
        .arg(Arg::with_name("input").help("source code").required(true))
        .arg(
            Arg::with_name("output")
                .help("Sets output path")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .get_matches();

    // Read source program
    let input_path = matches.value_of("input").unwrap();
    let mut input_file =
        File::open(input_path).unwrap_or_else(|_| panic!("{}: No such file", input_path));
    let mut source_program = String::new();
    input_file
        .read_to_string(&mut source_program)
        .unwrap_or_else(|_| panic!("{}: No such file", input_path));

    // lexical analysis
    let tokens = lexer::tokenize(&source_program);

    // syntactic analysis
    let (ast, errs) = parser::parse(tokens);

    if errs.is_empty() {
        let mut ast = ast.unwrap();
        // symbol resolution
        symbol::resolve_symbols(&mut ast.lines);
        let unresolved_symbols = symbol::check_unresolve_symbols(&ast.lines);
        if !unresolved_symbols.is_empty() {
            eprintln!("Unresolved symbols found");
            for unresolved_symbol in unresolved_symbols {
                eprintln!("- {unresolved_symbol}");
            }
            std::process::exit(1);
        }

        // Set a binary file name
        let output_path = if let Some(output_file_name) = matches.value_of("output") {
            PathBuf::from(output_file_name)
        } else {
            let mut output_path = PathBuf::from(input_path);
            output_path.set_extension("b");
            output_path
        };

        // Open a binary file
        let mut file = match File::create(&output_path) {
            Err(why) => panic!("{}: {why}", output_path.display()),
            Ok(file) => file,
        };

        // Write a binary file
        let code = codegen::generate(&ast.lines);
        write!(file, "MM {}", ast.title).unwrap();
        for (a, c) in code.iter() {
            write!(file, "\n{a:04X}  {c:04X}").unwrap();
        }
    } else {
        for err in errs {
            Report::build(ReportKind::Error, input_path, err.span().start)
                .with_message("Unexpected token")
                .with_label(Label::new((input_path, err.span())).with_message(format!(
                    "Unexpected token {}",
                    &source_program[err.span()].fg(Color::Red)
                )))
                .finish()
                .print((input_path, Source::from(&source_program)))
                .unwrap();
        }
    }
}
