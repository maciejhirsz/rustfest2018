use chapter_1::Token;

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
    pub body: Vec<Expression>
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operator {
    /// '+'
    Add,

    /// '-'
    Subtract,

    /// '*'
    Multiply,

    /// '/'
    Divide,

    /// '='
    Assign,
}

impl Operator {
    pub fn binding_power(&self) -> u8 {
        match self {
            Operator::Add |
            Operator::Subtract => 2,
            Operator::Multiply |
            Operator::Divide => 3,
            Operator::Assign => 1,
        }
    }
}

impl<'source> From<Token<'source>> for Operator {
    fn from(token: Token<'source>) -> Operator {
        match token {
            Token::Add => Operator::Add,
            Token::Subtract => Operator::Subtract,
            Token::Multiply => Operator::Multiply,
            Token::Divide => Operator::Divide,
            Token::Assign => Operator::Assign,
            _ => panic!("Not an operator!"),
        }
    }
}

// It tends to be a good practice to pack `struct`s
// into `enum` variants, instead of having fields on
// said variants as such.
//
// Ideally Rust would allow us to use things like
// `Expression::BinaryExpression` as types, but we
// are not there yet :).
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Number(Number),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Identifier {
    pub identifier: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub number: u64,
}

// This is mostly useful for things like `-5`
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Operator,
    pub operand: Box<Expression>,
}

// Should cover both assignment and addition
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

// Converters below
impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Expression {
        Expression::Identifier(value)
    }
}

impl From<Number> for Expression {
    fn from(value: Number) -> Expression {
        Expression::Number(value)
    }
}

impl From<UnaryExpression> for Expression {
    fn from(value: UnaryExpression) -> Expression {
        Expression::UnaryExpression(value)
    }
}

impl From<BinaryExpression> for Expression {
    fn from(value: BinaryExpression) -> Expression {
        Expression::BinaryExpression(value)
    }
}
