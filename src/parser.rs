pub mod lexer;
// pub mod tokenizer;

    use std::boxed::Box;
    use std::io;
    use std::str;
    // use lexer::Scanner;

    pub fn parse_js_reader(input_reader: &mut Box<io::Read>, output_writer: &mut Box<io::Write>) {
        let mut buffer = String::new();

        fn foo() {}

        // read the whole file
        let result = input_reader.read_to_string(&mut buffer);
        println!("{}", buffer);
        let scanner = super::parser::lexer::Scanner::new(buffer,&foo);

    }

    pub fn parse_js_buf(input_buf: Vec<u8>, output_buf:Vec<u8>) {
    }

    #[cfg(test)]
    mod bansuriTests {
        #[test]
        fn parse_test() {
        }
    }