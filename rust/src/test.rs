use crate::lexer::{Lexeme, Lexer};
use crate::*;
use std::process::Command;

use crate::parser::Parser;

fn parse_file(input_filename: &str) {
    let input_filename = "mud_tests/".to_owned() + input_filename;

    let file = fs::read(input_filename).expect("Unable to open file!");
    let mut parser = Parser::new(file);

    println!("{:?}", parser.parse().unwrap());
}

fn lex_file(input_filename: &str) {
    let input_filename = "mud_tests/".to_owned() + input_filename;

    let file = fs::read(input_filename).expect("Unable to open file!");
    let mut lexer = Lexer::new(file);
    // println!("{:?}", parser.parse().unwrap());

    loop {
        let lexeme = lexer.next();
        println!("{:?}", lexeme);
        if let Ok(Lexeme::Eof) = lexeme {
            break;
        }
    }
}

fn test_compile(test_name: &str){
    let input_filepath = "mud_tests/".to_string() + test_name;
    compile_file(&input_filepath, "");
}

fn test_transpile(test_name: &str){
    let input_filepath = "mud_tests/".to_string() + test_name;
    transpile_file(&input_filepath);
}


fn test_run(test_name: &str, expected_out: Option<&str>){
    let input_filepath = "mud_tests/".to_string() + test_name;

    transpile_file(&input_filepath);
    let output_filename: String = test_name.split(".").take(1).collect();
    let output = Command::new("./".to_string() +
                              &"mud_tests/" + &output_filename + &".exe")
        .output()
        .expect("Failed to run program");

    if !output.status.success(){
        dbg!(&output.status);
    }

    println!("run error: {}", String::from_utf8_lossy(&output.stderr));
    println!("run output: {}", String::from_utf8_lossy(&output.stdout));

    if let Some(expected_out) = expected_out{
        assert_eq!(expected_out, String::from_utf8_lossy(&output.stdout).replace("\r", ""));
    }

    assert!(
        output.status.success(),
        "The transpiled code crashed"
    );
}


#[test]
fn add_mul() {
    let filename = "add_mul.mud";
    test_compile(filename);
    test_transpile(filename);
    test_run(filename, None);
}

#[test]
fn sequence() {
    let filename = "sequence.mud";
    test_compile(filename);
}

// #[test]
// fn identifiers(){
//     test_compile("identifiers.mud");
// }

#[test]
fn assignment(){
    lex_file("assignment.mud");
    parse_file("assignment.mud");
    test_compile("assignment.mud");
}

#[test]
fn print(){
    lex_file("print.mud");
    test_compile("print.mud");
    test_run("print.mud", Some("5"));
}

#[test]
fn scope(){
    lex_file("scope.mud");
    test_compile("scope.mud");
}

#[test]
fn run_if_else() {
    let filename = "if_else.mud";
    test_transpile(filename);
}

#[test]
fn run_functions() {
    let filename = "functions.mud";
    test_transpile(filename);
}

#[test]
fn run_while() {
    let filename = "while.mud";
    test_compile(filename);
    test_run(filename, Some("109876543210"));
}

#[test]
fn string_literal() {
    let filename = "str_literal.mud";
    lex_file(filename);
    parse_file(filename);
    test_run(filename, Some("testcatcat"));
}

#[test]
fn operators(){
    let filename = "operators.mud";
    test_run(filename, Some("Passed"));
}

#[test]
fn pointer(){
    let filename = "pointer.mud";
    lex_file(filename);
    parse_file(filename);
    test_run(filename, Some("67"));
}

#[test]
fn char(){
    let filename = "char.mud";
    test_run(filename, Some("A"));
}

#[test]
fn comment(){
    let filename = "comment.mud";
    lex_file(filename);
    test_run(filename, Some("print this\nand this\n"));
}

#[test]
fn r#struct(){
    let filename = "struct.mud";
    // lex_file(filename);
    parse_file(filename);
    test_run(filename, Some("tom the cat is 7 years old.\n"));
}

#[test]
fn read_file(){
    let filename = "read_file.mud";
    test_run(filename, Some("some text\n"))
}

#[test]
fn r#const(){
    let filename = "const.mud";
    test_run(filename, Some("100"))
}

#[test]
fn casting(){
    let filename = "casting.mud";
    test_run(filename, Some("42"));
}
