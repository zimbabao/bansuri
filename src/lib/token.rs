use std::borrow::{Cow, ToOwned};

pub mod token {
  pub enum Token<'a> {
    BooleanLiteral(Cow<T, str>, name: ),
    EOF(Cow<'a, char>),
    Identifier(Cow<'a, str>),
    Keyword(Cow<'a, str>),
    NullLiteral(Cow<'a, str>),
    NumericLiteral(Cow<'a, str>),
    Punctuator(Cow<'a, str>),
    StringLiteral(Cow<'a, str>),
    RegularExpression(Cow<'a, str>),
    Template(Cow<'a, str>),
  }
}