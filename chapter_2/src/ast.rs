pub struct Program {
    pub statements: Vec<Expression>
}

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
pub enum Expression {
    Identifier(String),
    Number(i32),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
}

// This is mostly useful for things like `-5`
pub struct UnaryExpression {
    operator: Operator,
    operand: Box<Expression>,
}

// Should cover both assignment and addition
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

// Converters below
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
