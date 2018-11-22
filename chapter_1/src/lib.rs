#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Identifier,
    Number,
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
    Semicolon,
}

/// This struct needs some fields!
pub struct Lexer;

impl Lexer {
    /// It also needs some code
    pub fn new(source: &str) -> Self {
        Lexer
    }
}

/// We will also use the `Iterator` trait from the
/// standard library for our Lexer.
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        None
    }
}

#[cfg(test)]
#[test]
fn test() {
    let source = "four = 2 + 2; omg = 1 / 0;";

    let expect = &[
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Add,
        Token::Number,
        Token::Semicolon,
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Divide,
        Token::Number,
        Token::Semicolon,
    ];

    // Create an iterator of Tokens out of the slice here.
    let expect = expect.iter().cloned();
    let lexer = Lexer::new(source);

    // We can use the `eq` method of the `Iterator` trait
    // to check that they are equal
    assert!(lexer.eq(expect));
}
