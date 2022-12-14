pub struct DefineStatement(pub String, pub Box<Expression>);

pub struct FunctionExpression(pub Box<Vec<String>>, pub Box<Expression>);

pub struct IfExpression(
    pub Box<Expression>,
    pub Box<Expression>,
    pub Box<Expression>,
);

pub enum Statement {
    Expression(Box<Expression>),
    DefineStatement(Box<DefineStatement>),
    PrintStatement(Box<PrintStatement>),
}

pub enum PrintStatement {
    PrintNumber(Box<Expression>),
    PrintBoolean(Box<Expression>),
}

pub enum FunctionCall {
    ExpressionCall(Box<FunctionExpression>, Vec<Box<Expression>>),
    NameCall(String, Vec<Box<Expression>>),
}

pub enum Expression {
    Number(i32),
    Boolean(bool),
    Variable(String),
    NumOperate(Box<NumOperator>),
    LogicalOperate(Box<LogicalOperator>),
    FunctionExpression(Box<FunctionExpression>),
    FunctionCall(Box<FunctionCall>),
    IfExpression(Box<IfExpression>),
}

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

pub enum LogicalOperator {
    And(Box<Expression>, Vec<Box<Expression>>),
    Or(Box<Expression>, Vec<Box<Expression>>),
    Not(Box<Expression>),
}
