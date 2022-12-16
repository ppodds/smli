use std::collections::HashMap;

use crate::ast;

#[cfg(test)]
mod tests {
    use crate::{ast, interpreter};

    #[test]
    fn to_num_with_number() {
        let interpreter = interpreter::Interpreter::new();
        assert_eq!(interpreter.to_num(&interpreter::Value::Number(10)), Ok(10));
        assert_eq!(interpreter.to_num(&interpreter::Value::Number(0)), Ok(0));
        assert_eq!(
            interpreter.to_num(&interpreter::Value::Number(-10)),
            Ok(-10)
        );
    }

    #[test]
    fn to_num_with_other_type() {
        let interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .to_num(&interpreter::Value::Boolean(false))
            .is_err());
    }

    #[test]
    fn to_bool_with_bool() {
        let interpreter = interpreter::Interpreter::new();
        for b in vec![true, false] {
            assert_eq!(interpreter.to_bool(&interpreter::Value::Boolean(b)), Ok(b));
        }
    }

    #[test]
    fn to_bool_with_other_type() {
        let interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .to_bool(&interpreter::Value::Number(10))
            .is_err());
    }

    #[test]
    fn evaluate_number() {
        let interpreter = interpreter::Interpreter::new();
        assert_eq!(
            interpreter.eval_expression(&Box::new(ast::Expression::Number(10)), None),
            Ok(interpreter::Value::Number(10))
        );
    }

    #[test]
    fn evaluate_bool() {
        let interpreter = interpreter::Interpreter::new();
        assert_eq!(
            interpreter.eval_expression(&Box::new(ast::Expression::Boolean(true)), None),
            Ok(interpreter::Value::Boolean(true))
        );
    }

    #[test]
    fn evaluate_number_operate_plus() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))],
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                    ],
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Number(3)));
        assert_eq!(result2, Ok(interpreter::Value::Number(6)));
    }

    #[test]
    fn evaluate_number_operate_minus() {
        let interpreter = interpreter::Interpreter::new();
        let result = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Minus(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2)),
                ),
            ))),
            None,
        );
        assert_eq!(result, Ok(interpreter::Value::Number(-1)));
    }

    #[test]
    fn evaluate_number_operate_multiply() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Multiply(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))],
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Multiply(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                    ],
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Number(2)));
        assert_eq!(result2, Ok(interpreter::Value::Number(6)));
    }

    #[test]
    fn evaluate_number_operate_divide() {
        let interpreter = interpreter::Interpreter::new();
        let result = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Divide(
                    Box::new(ast::Expression::Number(5)),
                    Box::new(ast::Expression::Number(2)),
                ),
            ))),
            None,
        );
        assert_eq!(result, Ok(interpreter::Value::Number(2)));
    }

    #[test]
    fn evaluate_number_operate_modulus() {
        let interpreter = interpreter::Interpreter::new();
        let result = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Modulus(
                    Box::new(ast::Expression::Number(5)),
                    Box::new(ast::Expression::Number(2)),
                ),
            ))),
            None,
        );
        assert_eq!(result, Ok(interpreter::Value::Number(1)));
    }

    #[test]
    fn evaluate_number_operate_greater() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Greater(
                    Box::new(ast::Expression::Number(5)),
                    Box::new(ast::Expression::Number(2)),
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Greater(
                    Box::new(ast::Expression::Number(2)),
                    Box::new(ast::Expression::Number(5)),
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result2, Ok(interpreter::Value::Boolean(false)));
    }

    #[test]
    fn evaluate_number_operate_smaller() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Smaller(
                    Box::new(ast::Expression::Number(5)),
                    Box::new(ast::Expression::Number(2)),
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Smaller(
                    Box::new(ast::Expression::Number(2)),
                    Box::new(ast::Expression::Number(5)),
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Boolean(false)));
        assert_eq!(result2, Ok(interpreter::Value::Boolean(true)));
    }

    #[test]
    fn evaluate_number_operate_equal() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(1))],
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))],
                ),
            ))),
            None,
        );
        let result3 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(1)),
                        Box::new(ast::Expression::Number(1)),
                    ],
                ),
            ))),
            None,
        );
        let result4 = interpreter.eval_expression(
            &Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                    ],
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result2, Ok(interpreter::Value::Boolean(false)));
        assert_eq!(result3, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result4, Ok(interpreter::Value::Boolean(false)));
    }

    #[test]
    fn evaluate_logical_operate_and() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(true))],
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(false))],
                ),
            ))),
            None,
        );
        let result3 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![
                        Box::new(ast::Expression::Boolean(true)),
                        Box::new(ast::Expression::Boolean(true)),
                    ],
                ),
            ))),
            None,
        );
        let result4 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![
                        Box::new(ast::Expression::Boolean(false)),
                        Box::new(ast::Expression::Boolean(true)),
                    ],
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result2, Ok(interpreter::Value::Boolean(false)));
        assert_eq!(result3, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result4, Ok(interpreter::Value::Boolean(false)));
    }

    #[test]
    fn evaluate_logical_operate_or() {
        let interpreter = interpreter::Interpreter::new();
        let result1 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(true))],
                ),
            ))),
            None,
        );
        let result2 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(false)),
                    vec![Box::new(ast::Expression::Boolean(false))],
                ),
            ))),
            None,
        );
        let result3 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![
                        Box::new(ast::Expression::Boolean(true)),
                        Box::new(ast::Expression::Boolean(true)),
                    ],
                ),
            ))),
            None,
        );
        let result4 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(false)),
                    vec![
                        Box::new(ast::Expression::Boolean(false)),
                        Box::new(ast::Expression::Boolean(false)),
                    ],
                ),
            ))),
            None,
        );
        let result5 = interpreter.eval_expression(
            &Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(false)),
                    vec![
                        Box::new(ast::Expression::Boolean(true)),
                        Box::new(ast::Expression::Boolean(false)),
                    ],
                ),
            ))),
            None,
        );
        assert_eq!(result1, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result2, Ok(interpreter::Value::Boolean(false)));
        assert_eq!(result3, Ok(interpreter::Value::Boolean(true)));
        assert_eq!(result4, Ok(interpreter::Value::Boolean(false)));
        assert_eq!(result5, Ok(interpreter::Value::Boolean(true)));
    }

    #[test]
    fn evaluate_logical_operate_not() {
        let interpreter = interpreter::Interpreter::new();
        for t in vec![true, false] {
            let result = interpreter.eval_expression(
                &Box::new(ast::Expression::LogicalOperate(Box::new(
                    ast::LogicalOperator::Not(Box::new(ast::Expression::Boolean(t))),
                ))),
                None,
            );
            assert_eq!(result, Ok(interpreter::Value::Boolean(!t)));
        }
    }

    #[test]
    fn define_number_variable() {
        let mut interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .eval_statement(&ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("a".to_string()),
                    Box::new(ast::Expression::Number(1)),
                ),
            )))
            .is_ok());
        assert_eq!(
            interpreter.variables.get("a"),
            Some(&interpreter::Value::Number(1))
        );
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                None
            ),
            Ok(interpreter::Value::Number(1))
        );
    }

    #[test]
    fn define_boolean_variable() {
        let mut interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .eval_statement(&ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("a".to_string()),
                    Box::new(ast::Expression::Boolean(true)),
                ),
            )))
            .is_ok());
        assert_eq!(
            interpreter.variables.get("a"),
            Some(&interpreter::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                None
            ),
            Ok(interpreter::Value::Boolean(true))
        );
    }

    #[test]
    fn if_expression() {
        let interpreter = interpreter::Interpreter::new();
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::IfExpression(Box::new(ast::IfExpression(
                    Box::new(ast::Expression::Boolean(true)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2)),
                )))),
                None,
            ),
            Ok(interpreter::Value::Number(1))
        );
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::IfExpression(Box::new(ast::IfExpression(
                    Box::new(ast::Expression::Boolean(false)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2)),
                )))),
                None,
            ),
            Ok(interpreter::Value::Number(2))
        );
    }

    #[test]
    fn function_expression_call() {
        let interpreter = interpreter::Interpreter::new();
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::FunctionCall(Box::new(
                    ast::FunctionCall::ExpressionCall(
                        Box::new(ast::FunctionExpression(
                            Box::new(vec![Box::new("a".to_string()), Box::new("b".to_string())]),
                            Box::new(ast::Expression::NumOperate(Box::new(
                                ast::NumOperator::Plus(
                                    Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                                    vec![Box::new(ast::Expression::Variable(Box::new(
                                        "b".to_string(),
                                    )))],
                                ),
                            ))),
                        )),
                        vec![
                            Box::new(ast::Expression::Number(1)),
                            Box::new(ast::Expression::Number(1)),
                        ],
                    ),
                ))),
                None,
            ),
            Ok(interpreter::Value::Number(2))
        );
    }

    #[test]
    fn function_expression_call_with_wrong_args() {
        let interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .eval_expression(
                &Box::new(ast::Expression::FunctionCall(Box::new(
                    ast::FunctionCall::ExpressionCall(
                        Box::new(ast::FunctionExpression(
                            Box::new(vec![Box::new("a".to_string()), Box::new("b".to_string())]),
                            Box::new(ast::Expression::NumOperate(Box::new(
                                ast::NumOperator::Plus(
                                    Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                                    vec![Box::new(ast::Expression::Variable(Box::new(
                                        "b".to_string(),
                                    )))],
                                ),
                            ))),
                        )),
                        vec![
                            Box::new(ast::Expression::Number(1)),
                            Box::new(ast::Expression::Number(1)),
                            Box::new(ast::Expression::Number(1)),
                        ],
                    ),
                ))),
                None,
            )
            .is_err());
    }

    #[test]
    fn define_function_variable() {
        let mut interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .eval_statement(&ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("test".to_string()),
                    Box::new(ast::Expression::FunctionExpression(Box::new(
                        ast::FunctionExpression(
                            Box::new(vec![Box::new("a".to_string())]),
                            Box::new(ast::Expression::NumOperate(Box::new(
                                ast::NumOperator::Plus(
                                    Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                                    vec![Box::new(ast::Expression::Number(1))],
                                ),
                            ))),
                        ),
                    ))),
                ),
            )))
            .is_ok());
        assert_eq!(
            interpreter.eval_expression(
                &Box::new(ast::Expression::FunctionCall(Box::new(
                    ast::FunctionCall::NameCall(
                        Box::new("test".to_string()),
                        vec![Box::new(ast::Expression::Number(1))]
                    )
                ))),
                None
            ),
            Ok(interpreter::Value::Number(2))
        );
    }

    #[test]
    fn function_name_call_with_wrong_args() {
        let mut interpreter = interpreter::Interpreter::new();
        assert!(interpreter
            .eval_statement(&ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("test".to_string()),
                    Box::new(ast::Expression::FunctionExpression(Box::new(
                        ast::FunctionExpression(
                            Box::new(vec![Box::new("a".to_string())]),
                            Box::new(ast::Expression::NumOperate(Box::new(
                                ast::NumOperator::Plus(
                                    Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                                    vec![Box::new(ast::Expression::Number(1))],
                                ),
                            ))),
                        ),
                    ))),
                ),
            )))
            .is_ok());
        assert!(interpreter
            .eval_expression(
                &Box::new(ast::Expression::FunctionCall(Box::new(
                    ast::FunctionCall::NameCall(
                        Box::new("test".to_string()),
                        vec![
                            Box::new(ast::Expression::Number(1)),
                            Box::new(ast::Expression::Number(1)),
                        ],
                    ),
                ))),
                None,
            )
            .is_err());
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Number(i32),
    Boolean(bool),
    FunctionExpression(Box<ast::FunctionExpression>),
}

pub struct Interpreter {
    variables: Box<HashMap<String, Value>>,
}

fn validate_type(value: &Value, expected: &str) -> Result<(), String> {
    match value {
        Value::Number(_) => {
            if expected != "number" {
                return Err(format!("Type Error: Expect '{expected}' but got 'number'."));
            }
            Ok(())
        }
        Value::Boolean(_) => {
            if expected != "boolean" {
                return Err(format!(
                    "Type Error: Expect '{expected}' but got 'boolean'."
                ));
            }
            Ok(())
        }
        Value::FunctionExpression(_) => {
            if expected != "function" {
                return Err(format!(
                    "Type Error: Expect '{expected}' but got 'function'."
                ));
            }
            Ok(())
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: Box::new(HashMap::new()),
        }
    }

    pub fn run(&mut self, program: Box<Vec<Box<ast::Statement>>>) -> bool {
        for statement in program.iter() {
            match self.eval_statement(statement) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return false;
                }
            }
        }
        true
    }

    fn eval_statement(&mut self, statement: &ast::Statement) -> Result<(), String> {
        match statement {
            ast::Statement::Expression(expression) => {
                self.eval_expression(expression, None)?;
            }
            ast::Statement::DefineStatement(define_statement) => {
                self.eval_define_statement(define_statement)?;
            }
            ast::Statement::PrintStatement(print_statement) => {
                self.eval_print_statement(print_statement)?;
            }
        }
        Ok(())
    }

    fn eval_expression(
        &self,
        expression: &Box<ast::Expression>,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, String> {
        match &**expression {
            ast::Expression::Number(v) => Ok(Value::Number(*v)),
            ast::Expression::Boolean(v) => Ok(Value::Boolean(*v)),
            ast::Expression::Variable(v) => {
                // this is a function call, so check if the variable is a function argument first
                if variables.is_some() {
                    let variables = variables.unwrap();
                    if variables.contains_key(v.as_str()) {
                        return Ok(variables.get(v.as_str()).unwrap().clone());
                    }
                }
                Ok(self.get_variable(v.as_str())?.clone())
            }
            ast::Expression::NumOperate(op) => match &**op {
                ast::NumOperator::Plus(exp, exps) => {
                    let mut result = self.to_num(&self.eval_expression(&exp, variables)?)?;
                    for exp in exps {
                        result += self.to_num(&self.eval_expression(&exp, variables)?)?;
                    }
                    Ok(Value::Number(result))
                }
                ast::NumOperator::Minus(exp1, exp2) => Ok(Value::Number(
                    self.to_num(&self.eval_expression(&exp1, variables)?)?
                        - self.to_num(&self.eval_expression(&exp2, variables)?)?,
                )),
                ast::NumOperator::Multiply(exp, exps) => {
                    let mut result = self.to_num(&self.eval_expression(&exp, variables)?)?;
                    for exp in exps {
                        result *= self.to_num(&self.eval_expression(&exp, variables)?)?;
                    }
                    Ok(Value::Number(result))
                }
                ast::NumOperator::Divide(exp1, exp2) => Ok(Value::Number(
                    self.to_num(&self.eval_expression(&exp1, variables)?)?
                        / self.to_num(&self.eval_expression(&exp2, variables)?)?,
                )),
                ast::NumOperator::Modulus(exp1, exp2) => Ok(Value::Number(
                    self.to_num(&self.eval_expression(&exp1, variables)?)?
                        % self.to_num(&self.eval_expression(&exp2, variables)?)?,
                )),
                ast::NumOperator::Greater(exp1, exp2) => Ok(Value::Boolean(
                    self.to_num(&self.eval_expression(&exp1, variables)?)?
                        > self.to_num(&self.eval_expression(&exp2, variables)?)?,
                )),
                ast::NumOperator::Smaller(exp1, exp2) => Ok(Value::Boolean(
                    self.to_num(&self.eval_expression(&exp1, variables)?)?
                        < self.to_num(&self.eval_expression(&exp2, variables)?)?,
                )),
                ast::NumOperator::Equal(exp, exps) => {
                    let result = self.to_num(&self.eval_expression(&exp, variables)?)?;
                    for exp in exps {
                        if result != self.to_num(&self.eval_expression(&exp, variables)?)? {
                            return Ok(Value::Boolean(false));
                        }
                    }
                    Ok(Value::Boolean(true))
                }
            },
            ast::Expression::LogicalOperate(op) => match &**op {
                ast::LogicalOperator::And(exp, exps) => {
                    let result = self.to_bool(&self.eval_expression(&exp, variables)?)?;
                    for exp in exps {
                        if !(result && self.to_bool(&self.eval_expression(&exp, variables)?)?) {
                            return Ok(Value::Boolean(false));
                        }
                    }
                    Ok(Value::Boolean(result))
                }
                ast::LogicalOperator::Or(exp, exps) => {
                    let result = self.to_bool(&self.eval_expression(&exp, variables)?)?;
                    for exp in exps {
                        if result || self.to_bool(&self.eval_expression(&exp, variables)?)? {
                            return Ok(Value::Boolean(true));
                        }
                    }
                    Ok(Value::Boolean(result))
                }
                ast::LogicalOperator::Not(v) => Ok(Value::Boolean(
                    !self.to_bool(&self.eval_expression(&v, variables)?)?,
                )),
            },
            ast::Expression::FunctionExpression(e) => Ok(Value::FunctionExpression(e.clone())),
            ast::Expression::FunctionCall(fc) => match &**fc {
                ast::FunctionCall::ExpressionCall(fe, args) => {
                    let func_arg_len = fe.0.len();
                    let args_len = args.len();
                    if func_arg_len != args_len {
                        return Err(format!(
                            "Function expected {func_arg_len} arguments, but got {args_len}"
                        ));
                    }
                    let mut scoped_variables = HashMap::new();
                    // add new variables
                    let arg_names = &fe.0;
                    for i in 0..func_arg_len {
                        scoped_variables.insert(
                            arg_names[i].to_string(),
                            self.eval_expression(&args[i], variables)?,
                        );
                    }
                    Ok(self.eval_expression(&fe.1, Some(&scoped_variables))?)
                }
                ast::FunctionCall::NameCall(id, args) => {
                    let func = self.to_function(self.get_variable(id)?)?;
                    Ok(self.eval_expression(
                        &Box::new(ast::Expression::FunctionCall(Box::new(
                            ast::FunctionCall::ExpressionCall(func, args.to_vec()),
                        ))),
                        None,
                    )?)
                }
            },
            ast::Expression::IfExpression(v) => {
                let condition = self.to_bool(&self.eval_expression(&v.0, variables)?)?;
                if condition {
                    Ok(self.eval_expression(&v.1, variables)?)
                } else {
                    Ok(self.eval_expression(&v.2, variables)?)
                }
            }
        }
    }

    fn eval_define_statement(&mut self, statement: &ast::DefineStatement) -> Result<(), String> {
        self.variables.insert(
            statement.0.to_string(),
            self.eval_expression(&statement.1, None)?,
        );
        Ok(())
    }

    fn eval_print_statement(&self, statement: &ast::PrintStatement) -> Result<(), String> {
        match statement {
            ast::PrintStatement::PrintNumber(expression) => {
                println!("{}", self.to_num(&self.eval_expression(expression, None)?)?);
                Ok(())
            }
            ast::PrintStatement::PrintBoolean(expression) => {
                println!(
                    "{}",
                    self.to_bool(&self.eval_expression(expression, None)?)?
                );
                Ok(())
            }
        }
    }

    fn to_num(&self, value: &Value) -> Result<i32, String> {
        match value {
            Value::Number(v) => Ok(*v),
            _ => match validate_type(value, "number") {
                Err(e) => Err(e),
                _ => Err(String::from("This should not happen.")),
            },
        }
    }

    fn to_bool(&self, value: &Value) -> Result<bool, String> {
        match value {
            Value::Boolean(v) => Ok(*v),
            _ => match validate_type(value, "boolean") {
                Err(e) => Err(e),
                _ => Err(String::from("This should not happen.")),
            },
        }
    }

    fn to_function(&self, value: &Value) -> Result<Box<ast::FunctionExpression>, String> {
        match value {
            Value::FunctionExpression(v) => Ok(v.clone()),
            _ => match validate_type(value, "function") {
                Err(e) => Err(e),
                _ => Err(String::from("This should not happen.")),
            },
        }
    }

    fn get_variable(&self, id: &str) -> Result<&Value, String> {
        match self.variables.get(id) {
            Some(v) => Ok(v),
            None => Err(format!("'{id}' is not defined.")),
        }
    }
}
