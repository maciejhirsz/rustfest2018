use toolshed::list::List;

#[derive(PartialEq, Debug, Clone)]
pub struct Program<'ast> {
    pub body: List<'ast, Expression<'ast>>
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
pub enum Expression<'ast> {
    Identifier(Identifier<'ast>),
    Number(Number),
    UnaryExpression(UnaryExpression<'ast>),
    BinaryExpression(BinaryExpression<'ast>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Identifier<'ast> {
    pub identifier: &'ast str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub number: u64,
}

// This is mostly useful for things like `-5`
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpression<'ast> {
    pub operator: Operator,
    pub operand: &'ast Expression<'ast>,
}

// Should cover both assignment and addition
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression<'ast> {
    pub left: &'ast Expression<'ast>,
    pub operator: Operator,
    pub right: &'ast Expression<'ast>,
}

// Converters below
impl<'ast> From<Identifier<'ast>> for Expression<'ast> {
    fn from(value: Identifier<'ast>) -> Expression<'ast> {
        Expression::Identifier(value)
    }
}

impl<'ast> From<Number> for Expression<'ast> {
    fn from(value: Number) -> Expression<'ast> {
        Expression::Number(value)
    }
}

impl<'ast> From<UnaryExpression<'ast>> for Expression<'ast> {
    fn from(value: UnaryExpression<'ast>) -> Expression<'ast> {
        Expression::UnaryExpression(value)
    }
}

impl<'ast> From<BinaryExpression<'ast>> for Expression<'ast> {
    fn from(value: BinaryExpression<'ast>) -> Expression<'ast> {
        Expression::BinaryExpression(value)
    }
}
