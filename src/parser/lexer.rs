use std::i64;
use String;

use std::borrow::{Cow};

pub enum Token {
  BooleanLiteral(bool),
  EOF,
  Identifier(String),
  Keyword(String),
  NullLiteral,
  NumericLiteral(f64),
  Punctuator(String),
  StringLiteral(String),
  RegularExpression(String),
  Template(String),
}

struct RawToken {
  tokenType: Token,
  value: String,
  lineNumber: usize,
  lineStart: usize,
}

struct ScannerState {
  pub index: usize,
  pub lineNumber: usize,
  pub lineStart: usize,
}

pub struct Scanner {
  code: String, // Readonly
  curlyStack: Vec<char>,
  trackComment: bool,
  lineNumber: usize,
  lineStart: usize,
}

impl Scanner {
  pub fn new(code: String, errorHandler: &Fn()) -> Self {
    Scanner {
      code: code,
      curlyStack: Vec::<char>::new(),
      lineNumber: 0,
      lineStart: 0,
      trackComment: true,
    }
  }
}

//TODO: Some function declartions have incosistency
// Need to check why its like that and comment or
// make then consistent
trait ScannerTrait {
  // fn throwUnexpectedError(message: String);

  // fn tolerateUnexpectedToken(mnessage: String);

  // fn eof() -> bool;

  // fn skipSingleLineComment(offset: usize);

  // fn skipMultiLineComment();

  // fn scanComments();

  // fn isFutureReservedWord(w: String) -> bool;

  // fn isStrictModeReservedWord(w: String) -> bool;

  // fn isRestrictedWord(w: String) -> bool;

  // fn isKeyword(w: String) -> bool;

  // fn codePointAt(i: usize) -> u32;

  // fn scanHexEscape(prefix: String) -> String;

  // fn scanUnicodeCodePointEscape() -> String;

  // fn getIdentifier() -> String;

  // fn getComplexIdentifier() -> String;

  // fn octalToDecimal(ch: char) -> Option<i64>;

  // fn scanIdentifier() -> RawToken<'a>;

  // fn scanPuntuators() -> RawToken<'a>;

  // fn scanHexLiterals(start: usize) -> RawToken<'a>;

  // fn scanBinaryLiterals(start: usize) -> RawToken<'a>;

  // //TODO: Why this different from octal
  // fn scanOctalLiteral(prefix: String, start: usize) -> RawToken<'a>;

  // fn isImplicitOctalLiteral() -> bool;

  // fn scanNumericLiteral() -> RawToken<'a>;

  // fn scanLiteralString() -> RawToken<'a>;

  // fn scanTemplate() -> RawToken<'a>;

  // //TODO: Choose appropriate types
  // fn testRegExp(pattern: String, flags: String) -> Option<String>;

  // //TODO: Why this is public and others are private
  // fn scanRegExp() -> RawToken<'a>;

  fn lex() -> RawToken;
}

impl ScannerTrait for Scanner {
  fn lex() -> RawToken {
    return RawToken {
      lineNumber: 0,
      lineStart: 1,
      tokenType: Token::BooleanLiteral(true),
      value: String::from("dcjksdhks"),
    }
  }


}

// struct
// impl lexer::Scanner for lex

#[cfg(test)]
mod lexerTests {
    #[test]
    fn exploration() {
    }
}