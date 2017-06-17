use token;
use std::i64;

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

struct Scanner {
  code: String; // Readonly
  curlyStack: Vec<char>;
  trackComment: bool;
  lineNumber: usize;
  lineStart: usize;
}

trait Scanner {
  fn new(code: String, errorHandler: &Fn()) -> Self;
}