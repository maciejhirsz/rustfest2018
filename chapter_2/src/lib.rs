extern crate chapter_1;
extern crate pretty_assertions;

mod ast;

pub use chapter_1::{Lexer, Token};
pub use std::iter::Peekable;

pub use ast::*;

/// This struct needs some fields!
///
/// Try to incorporate the Lexer from the previous chapter inside here
pub struct Parser<'a> {
    pub lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// It also needs some code
    pub fn new(source: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(source).peekable(),
        }
    }

    /// A bit of a helper that will improve ergonomics
    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn parse_simple(&mut self) -> Result<Expression> {
        match self.lexer.peek().cloned() {
            Some(Token::Add) |
            Some(Token::Subtract) => {
                UnaryExpression::parse(self).map(Into::into)
            },
            Some(Token::Number(_)) => {
                Number::parse(self).map(Into::into)
            },
            Some(Token::Identifier(_)) => {
                Identifier::parse(self).map(Into::into)
            },
            _ => return Err(ParseError),
         }
    }

    pub fn parse_nested(&mut self, mut left: Expression, lbp: u8) -> Result<Expression> {
        loop {
            let operator = match self.lexer.peek().cloned() {
                Some(token) if token.is_operator() => {
                    Operator::from(token)
                },
                _ => return Ok(left),
            };

            let rbp = operator.binding_power();

            if lbp > rbp {
                break;
            }

            self.lexer.next();

            let right: Expression = self.parse_simple()?;
            let right = self.parse_nested(right, rbp)?;

            left = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }
}

/// Just a 0-size type for clarity, we could also use `Option` instead of `Result`
#[derive(Debug)]
pub struct ParseError;

pub type Result<T> = ::std::result::Result<T, ParseError>;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

// All impls below need proper code!

impl Parse for Program {
    fn parse(parser: &mut Parser) -> Result<Program> {
        let mut body = Vec::new();

        loop {
            match parser.lexer.peek() {
                Some(Token::Semicolon) => {
                    parser.lexer.next();

                    continue;
                },
                Some(_) => {
                    body.push(parser.parse()?);
                },
                None => break,
            }
        }

        Ok(Program {
            body
        })
    }
}

impl Parse for Identifier {
    fn parse(parser: &mut Parser) -> Result<Identifier> {
        match parser.lexer.next() {
            Some(Token::Identifier(slice)) => {
                Ok(Identifier {
                    identifier: slice.to_string(),
                })
            },
            _ => Err(ParseError)
        }
    }
}

impl Parse for Number {
    fn parse(parser: &mut Parser) -> Result<Number> {
        match parser.lexer.next() {
            Some(Token::Number(number)) => {
                Ok(Number {
                    number,
                })
            },
            _ => Err(ParseError)
        }
    }
}

impl Parse for Operator {
    fn parse(parser: &mut Parser) -> Result<Operator> {
        match parser.lexer.next() {
            Some(Token::Add) => Ok(Operator::Add),
            Some(Token::Subtract) => Ok(Operator::Subtract),
            Some(Token::Multiply) => Ok(Operator::Multiply),
            Some(Token::Divide) => Ok(Operator::Divide),
            Some(Token::Assign) => Ok(Operator::Assign),
            _ => Err(ParseError),
        }
    }
}

impl Parse for UnaryExpression {
    fn parse(parser: &mut Parser) -> Result<UnaryExpression> {
        let operator = parser.parse()?;
        let operand = Box::new(parser.parse()?);

        Ok(UnaryExpression {
            operator,
            operand,
        })
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Expression> {
        let left = parser.parse_simple()?;

         parser.parse_nested(left, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_identifier() {
        let source = "foo;";

        let ident: Identifier = Parser::new(source).parse().unwrap();

        assert_eq!(ident, Identifier {
            identifier: "foo".into(),
        });
    }

    #[test]
    fn parse_number() {
        let source = "42;";

        let num: Number = Parser::new(source).parse().unwrap();

        assert_eq!(num, Number {
            number: 42,
        });
    }

    #[test]
    fn parse_neg_number() {
        let source = "-42;";

        let unary: UnaryExpression = Parser::new(source).parse().unwrap();

        assert_eq!(unary, UnaryExpression {
            operator: Operator::Subtract,
            operand: Box::new(Number {
                number: 42,
            }.into()),
        });
    }

    #[test]
    fn parse_binary_expression() {
        let source = "5 * 3;";

        let binary: Expression = Parser::new(source).parse().unwrap();

        assert_eq!(binary, BinaryExpression {
            left: Box::new(Number {
                number: 5,
            }.into()),
            operator: Operator::Multiply,
            right: Box::new(Number {
                number: 3,
            }.into()),
        }.into());
    }

    #[test]
    fn all_together() {
        let source = "foo = 42; bar = 10 * foo + foo / 3;";

        let program: Program = Parser::new(source).parse().unwrap();
//
        // panic!("{:#?}", program);

        assert_eq!(program, Program {
            body: vec![
                BinaryExpression {
                    left: Box::new(Identifier {
                        identifier: "foo".into(),
                    }.into()),
                    operator: Operator::Assign,
                    right: Box::new(Number {
                        number: 42,
                    }.into()),
                }.into(),
                BinaryExpression {
                    left: Box::new(Identifier {
                        identifier: "bar".into(),
                    }.into()),
                    operator: Operator::Assign,
                    right: Box::new(BinaryExpression {
                        left: Box::new(BinaryExpression {
                            left: Box::new(Number {
                                number: 10,
                            }.into()),
                            operator: Operator::Multiply,
                            right: Box::new(Identifier {
                                identifier: "foo".into(),
                            }.into()),
                        }.into()),
                        operator: Operator::Add,
                        right: Box::new(BinaryExpression {
                            left: Box::new(Identifier {
                                identifier: "foo".into(),
                            }.into()),
                            operator: Operator::Divide,
                            right: Box::new(Number {
                                number: 3,
                            }.into()),
                        }.into()),
                    }.into())
                }.into(),
            ]
        });
    }
}
