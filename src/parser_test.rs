#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::grammer;
    #[test]
    fn test_num() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("1"),
            Ok(Box::new(ast::Expression::Number(1)))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("0"),
            Ok(Box::new(ast::Expression::Number(0)))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("-1"),
            Ok(Box::new(ast::Expression::Number(-1)))
        );
        assert!(grammer::ExpressionParser::new().parse("1.0").is_err());
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("#t"),
            Ok(Box::new(ast::Expression::Boolean(true)))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("#f"),
            Ok(Box::new(ast::Expression::Boolean(false)))
        );
    }

    #[test]
    fn test_num_op() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("(+ 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(+ 1 2 3 4)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                        Box::new(ast::Expression::Number(4))
                    ]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(- 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Minus(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(* 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Multiply(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(* 1 2 3 4)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Multiply(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                        Box::new(ast::Expression::Number(4))
                    ]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(/ 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Divide(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(mod 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Modulus(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(> 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Greater(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(< 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Smaller(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(= 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(= 1 2 3 4)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![
                        Box::new(ast::Expression::Number(2)),
                        Box::new(ast::Expression::Number(3)),
                        Box::new(ast::Expression::Number(4))
                    ]
                )
            ))))
        );
    }

    #[test]
    fn test_logical_op() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("(and #t #t)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(true))]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(and #t #t #f #f)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![
                        Box::new(ast::Expression::Boolean(true)),
                        Box::new(ast::Expression::Boolean(false)),
                        Box::new(ast::Expression::Boolean(false))
                    ]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(or #t #f)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(false))]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(or #t #f #f #t)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![
                        Box::new(ast::Expression::Boolean(false)),
                        Box::new(ast::Expression::Boolean(false)),
                        Box::new(ast::Expression::Boolean(true))
                    ]
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(not #t)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Not(Box::new(ast::Expression::Boolean(true)))
            ))))
        );
    }

    #[test]
    fn test_function_expression() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("(fun (a b) (+ a b))"),
            Ok(Box::new(ast::Expression::FunctionExpression(Box::new(
                ast::FunctionExpression(
                    Box::new(vec![Box::new("a".to_string()), Box::new("b".to_string())]),
                    Box::new(vec![]),
                    Box::new(ast::Expression::NumOperate(Box::new(
                        ast::NumOperator::Plus(
                            Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                            vec![Box::new(ast::Expression::Variable(Box::new(
                                "b".to_string()
                            )))]
                        )
                    )))
                )
            ))))
        );
    }

    #[test]
    fn test_function_call() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("((fun (x y z) (+ x y z)) 10 20 30)"),
            Ok(Box::new(ast::Expression::FunctionCall(Box::new(
                ast::FunctionCall::ExpressionCall(
                    Box::new(ast::FunctionExpression(
                        Box::new(vec![
                            Box::new("x".to_string()),
                            Box::new("y".to_string()),
                            Box::new("z".to_string())
                        ]),
                        Box::new(vec![]),
                        Box::new(ast::Expression::NumOperate(Box::new(
                            ast::NumOperator::Plus(
                                Box::new(ast::Expression::Variable(Box::new("x".to_string()))),
                                vec![
                                    Box::new(ast::Expression::Variable(Box::new("y".to_string()))),
                                    Box::new(ast::Expression::Variable(Box::new("z".to_string())))
                                ]
                            )
                        )))
                    )),
                    vec![
                        Box::new(ast::Expression::Number(10)),
                        Box::new(ast::Expression::Number(20)),
                        Box::new(ast::Expression::Number(30))
                    ]
                )
            ))))
        );
    }

    #[test]
    fn test_define_statement() {
        assert_eq!(
            grammer::StatementParser::new().parse("(define a 1)"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("a".to_string()),
                    Box::new(ast::Expression::Number(1))
                )
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new().parse("(define b #t)"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("b".to_string()),
                    Box::new(ast::Expression::Boolean(true))
                )
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new().parse("(define foo (fun (a b) (+ a b)))"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("foo".to_string()),
                    Box::new(ast::Expression::FunctionExpression(Box::new(
                        ast::FunctionExpression(
                            Box::new(vec![Box::new("a".to_string()), Box::new("b".to_string())]),
                            Box::new(vec![]),
                            Box::new(ast::Expression::NumOperate(Box::new(
                                ast::NumOperator::Plus(
                                    Box::new(ast::Expression::Variable(Box::new("a".to_string()))),
                                    vec![Box::new(ast::Expression::Variable(Box::new(
                                        "b".to_string()
                                    )))]
                                )
                            )))
                        )
                    )))
                )
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new()
                .parse("(define fact (fun (n) (if (< n 3) n (* n (fact (- n 1))))))"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("fact".to_string()),
                    Box::new(ast::Expression::FunctionExpression(Box::new(
                        ast::FunctionExpression(
                            Box::new(vec![Box::new("n".to_string())]),
                            Box::new(vec![]),
                            Box::new(ast::Expression::IfExpression(Box::new(ast::IfExpression(
                                Box::new(ast::Expression::NumOperate(Box::new(
                                    ast::NumOperator::Smaller(
                                        Box::new(ast::Expression::Variable(Box::new(
                                            "n".to_string()
                                        ))),
                                        Box::new(ast::Expression::Number(3))
                                    )
                                ))),
                                Box::new(ast::Expression::Variable(Box::new("n".to_string()))),
                                Box::new(ast::Expression::NumOperate(Box::new(
                                    ast::NumOperator::Multiply(
                                        Box::new(ast::Expression::Variable(Box::new(
                                            "n".to_string()
                                        ))),
                                        vec![Box::new(ast::Expression::FunctionCall(Box::new(
                                            ast::FunctionCall::NameCall(
                                                Box::new("fact".to_string()),
                                                vec![Box::new(ast::Expression::NumOperate(
                                                    Box::new(ast::NumOperator::Minus(
                                                        Box::new(ast::Expression::Variable(
                                                            Box::new("n".to_string())
                                                        )),
                                                        Box::new(ast::Expression::Number(1))
                                                    ))
                                                ))]
                                            )
                                        )))]
                                    )
                                )))
                            ))))
                        )
                    )))
                )
            ))))
        );
    }

    #[test]
    fn test_expression_statement() {
        assert_eq!(
            grammer::StatementParser::new().parse("1"),
            Ok(Box::new(ast::Statement::Expression(Box::new(
                ast::Expression::Number(1)
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new().parse("#t"),
            Ok(Box::new(ast::Statement::Expression(Box::new(
                ast::Expression::Boolean(true)
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new().parse("(+ 1 2)"),
            Ok(Box::new(ast::Statement::Expression(Box::new(
                ast::Expression::NumOperate(Box::new(ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )))
            ))))
        );
    }

    #[test]
    fn test_print_statement() {
        assert_eq!(
            grammer::StatementParser::new().parse("(print-num 1)"),
            Ok(Box::new(ast::Statement::PrintStatement(Box::new(
                ast::PrintStatement::PrintNumber(Box::new(ast::Expression::Number(1)))
            ))))
        );
        assert_eq!(
            grammer::StatementParser::new().parse("(print-bool #t)"),
            Ok(Box::new(ast::Statement::PrintStatement(Box::new(
                ast::PrintStatement::PrintBoolean(Box::new(ast::Expression::Boolean(true)))
            ))))
        );
    }

    #[test]
    fn test_if_expression() {
        assert_eq!(
            grammer::ExpressionParser::new().parse("(if #t 1 2)"),
            Ok(Box::new(ast::Expression::IfExpression(Box::new(
                ast::IfExpression(
                    Box::new(ast::Expression::Boolean(true)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            grammer::ExpressionParser::new().parse("(if #f 1 2)"),
            Ok(Box::new(ast::Expression::IfExpression(Box::new(
                ast::IfExpression(
                    Box::new(ast::Expression::Boolean(false)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
    }
}
