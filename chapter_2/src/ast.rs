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
