extern crate toolshed;

mod ast;

pub use ast::*;

/// Again, we have nothing here yet. Remove PhantomData with a reference to an Arena.
pub struct Parser<'ast> {
    phantom: ::std::marker::PhantomData<Program<'ast>>,
}

impl<'ast> Parser<'ast> {
    /// It also needs some code
    pub fn new(source: &str) -> Self {
        Parser {
            phantom: ::std::marker::PhantomData,
        }
    }

    /// A bit of a helper that will improve ergonomics
    pub fn parse<T: Parse<'ast>>(&mut self) -> Result<T> {
        T::parse(self)
    }
}

/// Just a 0-size type for clarity, we could also use `Option` instead of `Result`
#[derive(Debug)]
pub struct ParseError;

pub type Result<T> = ::std::result::Result<T, ParseError>;

pub trait Parse<'ast>: Sized + 'ast {
    fn parse(parser: &mut Parser<'ast>) -> Result<Self>;
}

// All impls below need proper code!

impl<'ast> Parse<'ast> for Program<'ast> {
    fn parse(parser: &mut Parser<'ast>) -> Result<Program<'ast>> {
        Err(ParseError)
    }
}

impl<'ast> Parse<'ast> for Identifier<'ast> {
    fn parse(parser: &mut Parser<'ast>) -> Result<Identifier<'ast>> {
        Err(ParseError)
    }
}

impl<'ast> Parse<'ast> for Number {
    fn parse(parser: &mut Parser<'ast>) -> Result<Number> {
        Err(ParseError)
    }
}

impl<'ast> Parse<'ast> for UnaryExpression<'ast> {
    fn parse(parser: &mut Parser<'ast>) -> Result<UnaryExpression<'ast>> {
        Err(ParseError)
    }
}

impl<'ast> Parse<'ast> for BinaryExpression<'ast> {
    fn parse(parser: &mut Parser<'ast>) -> Result<BinaryExpression<'ast>> {
        Err(ParseError)
    }
}

impl<'ast> Parse<'ast> for Expression<'ast> {
    fn parse(parser: &mut Parser<'ast>) -> Result<Expression<'ast>> {
        // Suggested way to do this:
        //
        // match next_token {
        //     Token::_____ => ______::parse(parser),
        //
        //     ...
        //
        //     _ => Err(ParseError),
        // }

        Err(ParseError)
    }
}
