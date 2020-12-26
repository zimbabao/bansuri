pub mod lexer;

use std::boxed::Box;
use std::io;
// use lexer::Scanner;

pub fn parse_js_reader(input_reader: &mut Box<dyn io::Read>, output_writer: &mut Box<dyn io::Write>) {
    let mut buffer = String::new();

    fn error_handler() {}

    // read the whole file
    let result = input_reader.read_to_string(&mut buffer);
    println!("{}", buffer);
    let scanner = super::parser::lexer::Scanner::new(buffer,&error_handler);
}

pub fn parse_js_buf(input_buf: Vec<u8>, output_buf:Vec<u8>) {
}

#[cfg(test)]
mod bansuri_tests {
    #[test]
    fn parse_test() {
    }
}