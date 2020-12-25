use token;
use std::i64;

pub mod lexer {
  struct RawToken {
    type: Token;
    value: String;
    lineNumber: usize;
    lineStart: usize;
  }

  struct ScannerState {
    pub index: usize;
    pub lineNumber: usize;
    pub lineStart: usize;
  }

  pub struct Scanner {
    code: String; // Readonly
    curlyStack: Vec<char>;
    trackComment: bool;
    lineNumber: usize;
    lineStart: usize;
  }

  //TODO: Some function declartions have incosistency
  // Need to check why its like that and comment or
  // make then consistent
  trait Scanner {
    pub fn new(code: String, errorHandler: &Fn()) -> Self;

    fn throwUnexpectedError(message: String);

    fn tolerateUnexpectedToken(mnessage: String);

    pub fn eof() -> bool;

    fn skipSingleLineComment(offset: usize);

    fn skipMultiLineComment();

    pub fn scanComments();

    pub fn isFutureReservedWord(w: String) -> bool;

    pub fn isStrictModeReservedWord(w: String) -> bool;

    pub fn isRestrictedWord(w: String) -> bool;

    fn isKeyword(w: String) -> bool;

    fn codePointAt(i: usize) -> u32;

    fn scanHexEscape(prefix: String) -> String;

    fn scanUnicodeCodePointEscape() -> String;

    fn getIdentifier() -> String;

    fn getComplexIdentifier() -> String;

    fn octalToDecimal(ch: char) -> Option<i64>;

    fn scanIdentifier() -> RawToken;

    fn scanPuntuators() -> RawToken;

    fn scanHexLiterals(start: usize) -> RawToken;

    fn scanBinaryLiterals(start: usize) -> RawToken;

    //TODO: Why this different from octal
    fn scanOctalLiteral(prefix: String, start: usize) -> RawToken;

    fn isImplicitOctalLiteral() -> bool;

    fn scanNumericLiteral() -> RawToken;

    fn scanLiteralString() -> RawToken;

    fn scanTemplate() -> RawToken;

    //TODO: Choose appropriate types
    fn testRegExp(pattern: String, flags: String) -> Option<String>;

    //TODO: Why this is public and others are private
    pub fn scanRegExp() -> RawToken;

    pub fn lex() -> RawToken;
  }

  impl lexer::Scanner for Scanner {
    pub fn lex() -> RowToken {

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