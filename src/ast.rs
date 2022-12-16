#[derive(Debug, PartialEq)]
pub struct DefineStatement(pub Box<String>, pub Box<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression(pub Box<Vec<Box<String>>>, pub Box<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression(
    pub Box<Expression>,
    pub Box<Expression>,
    pub Box<Expression>,
);

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Box<Expression>),
    DefineStatement(Box<DefineStatement>),
    PrintStatement(Box<PrintStatement>),
}

#[derive(Debug, PartialEq)]
pub enum PrintStatement {
    PrintNumber(Box<Expression>),
    PrintBoolean(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCall {
    ExpressionCall(Box<FunctionExpression>, Vec<Box<Expression>>),
    NameCall(Box<String>, Vec<Box<Expression>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(i32),
    Boolean(bool),
    Variable(Box<String>),
    NumOperate(Box<NumOperator>),
    LogicalOperate(Box<LogicalOperator>),
    FunctionExpression(Box<FunctionExpression>),
    FunctionCall(Box<FunctionCall>),
    IfExpression(Box<IfExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumOperator {
    Plus(Box<Expression>, Vec<Box<Expression>>),
    Minus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Vec<Box<Expression>>),
    Divide(Box<Expression>, Box<Expression>),
    Modulus(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    Smaller(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Vec<Box<Expression>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    And(Box<Expression>, Vec<Box<Expression>>),
    Or(Box<Expression>, Vec<Box<Expression>>),
    Not(Box<Expression>),
}
