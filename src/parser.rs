#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::smli;
    #[test]
    fn test_num() {
        assert_eq!(
            smli::ExpressionParser::new().parse("1"),
            Ok(Box::new(ast::Expression::Number(1)))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("0"),
            Ok(Box::new(ast::Expression::Number(0)))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("-1"),
            Ok(Box::new(ast::Expression::Number(-1)))
        );
        assert!(smli::ExpressionParser::new().parse("1.0").is_err());
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            smli::ExpressionParser::new().parse("#t"),
            Ok(Box::new(ast::Expression::Boolean(true)))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("#f"),
            Ok(Box::new(ast::Expression::Boolean(false)))
        );
    }

    #[test]
    fn test_num_op() {
        assert_eq!(
            smli::ExpressionParser::new().parse("(+ 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Plus(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(+ 1 2 3 4)"),
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
            smli::ExpressionParser::new().parse("(- 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Minus(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(* 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Multiply(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(* 1 2 3 4)"),
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
            smli::ExpressionParser::new().parse("(/ 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Divide(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(mod 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Modulus(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(> 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Greater(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(< 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Smaller(
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(= 1 2)"),
            Ok(Box::new(ast::Expression::NumOperate(Box::new(
                ast::NumOperator::Equal(
                    Box::new(ast::Expression::Number(1)),
                    vec![Box::new(ast::Expression::Number(2))]
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(= 1 2 3 4)"),
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
            smli::ExpressionParser::new().parse("(and #t #t)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::And(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(true))]
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(and #t #t #f #f)"),
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
            smli::ExpressionParser::new().parse("(or #t #f)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Or(
                    Box::new(ast::Expression::Boolean(true)),
                    vec![Box::new(ast::Expression::Boolean(false))]
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(or #t #f #f #t)"),
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
            smli::ExpressionParser::new().parse("(not #t)"),
            Ok(Box::new(ast::Expression::LogicalOperate(Box::new(
                ast::LogicalOperator::Not(Box::new(ast::Expression::Boolean(true)))
            ))))
        );
    }

    #[test]
    fn test_function_expression() {
        assert_eq!(
            smli::ExpressionParser::new().parse("(fun (a b) (+ a b))"),
            Ok(Box::new(ast::Expression::FunctionExpression(Box::new(
                ast::FunctionExpression(
                    Box::new(vec![Box::new("a".to_string()), Box::new("b".to_string())]),
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
            smli::ExpressionParser::new().parse("((fun (x y z) (+ x y z)) 10 20 30)"),
            Ok(Box::new(ast::Expression::FunctionCall(Box::new(
                ast::FunctionCall::ExpressionCall(
                    Box::new(ast::FunctionExpression(
                        Box::new(vec![
                            Box::new("x".to_string()),
                            Box::new("y".to_string()),
                            Box::new("z".to_string())
                        ]),
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
            smli::StatementParser::new().parse("(define a 1)"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("a".to_string()),
                    Box::new(ast::Expression::Number(1))
                )
            ))))
        );
        assert_eq!(
            smli::StatementParser::new().parse("(define b #t)"),
            Ok(Box::new(ast::Statement::DefineStatement(Box::new(
                ast::DefineStatement(
                    Box::new("b".to_string()),
                    Box::new(ast::Expression::Boolean(true))
                )
            ))))
        );
    }

    #[test]
    fn test_expression_statement() {
        assert_eq!(
            smli::StatementParser::new().parse("1"),
            Ok(Box::new(ast::Statement::Expression(Box::new(
                ast::Expression::Number(1)
            ))))
        );
        assert_eq!(
            smli::StatementParser::new().parse("#t"),
            Ok(Box::new(ast::Statement::Expression(Box::new(
                ast::Expression::Boolean(true)
            ))))
        );
        assert_eq!(
            smli::StatementParser::new().parse("(+ 1 2)"),
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
            smli::StatementParser::new().parse("(print-num 1)"),
            Ok(Box::new(ast::Statement::PrintStatement(Box::new(
                ast::PrintStatement::PrintNumber(Box::new(ast::Expression::Number(1)))
            ))))
        );
        assert_eq!(
            smli::StatementParser::new().parse("(print-bool #t)"),
            Ok(Box::new(ast::Statement::PrintStatement(Box::new(
                ast::PrintStatement::PrintBoolean(Box::new(ast::Expression::Boolean(true)))
            ))))
        );
    }

    #[test]
    fn test_if_expression() {
        assert_eq!(
            smli::ExpressionParser::new().parse("(if #t 1 2)"),
            Ok(Box::new(ast::Expression::IfExpression(Box::new(
                ast::IfExpression(
                    Box::new(ast::Expression::Boolean(true)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
        assert_eq!(
            smli::ExpressionParser::new().parse("(if #f 1 2)"),
            Ok(Box::new(ast::Expression::IfExpression(Box::new(
                ast::IfExpression(
                    Box::new(ast::Expression::Boolean(false)),
                    Box::new(ast::Expression::Number(1)),
                    Box::new(ast::Expression::Number(2))
                )
            ))))
        );
    }

    #[test]
    fn test_program() {
        assert!(smli::ProgramParser::new().parse("(+)").is_err());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num 1)
(print-num 2)
(print-num 3)
(print-num 4)


"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num 0)
(print-num -123)
(print-num 456)

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(+ 1 2 3)
(* 4 5 6)

(print-num (+ 1 (+ 2 3 4) (* 4 5 6) (/ 8 3) (mod 10 3)))

(print-num (mod 10 4))

(print-num (- (+ 1 2) 4))

(print-num -256)

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num (mod 10 (+ 1 2)))

(print-num (* (/ 1 2) 4))

(print-num (- (+ 1 2 3 (- 4 5) 6 (/ 7 8) (mod 9 10))
              11))


"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-bool #t)
(print-bool #f)

(print-bool (and #t #f))
(print-bool (and #t #t))

(print-bool (or #t #f))
(print-bool (or #f #f))

(print-bool (not #t))
(print-bool (not #f))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-bool (or #t #t #f))
(print-bool (or #f (and #f #t) (not #f)))
(print-bool (and #t (not #f) (or #f #t) (and #t (not #t))))


"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num (if #t 1 2))

(print-num (if #f 1 2))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num (if (< 1 2) (+ 1 2 3) (* 1 2 3 4 5)))

(print-num (if (= 9 (* 2 5))
               0
               (if #t 1 2)))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(define x 1)

(print-num x)

(define y (+ 1 2 3))

(print-num y)

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(define a (* 1 2 3 4))

(define b (+ 10 -5 -2 -1))

(print-num (+ a b))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(print-num
  ((fun (x) (+ x 1)) 3))

(print-num
  ((fun (a b) (+ a b)) 4 5))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(define x 0)

(print-num
  ((fun (x y z) (+ x (* y z))) 10 20 30))


(print-num x)

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(define foo
  (fun (a b c) (+ a b (* b c))))

(print-num (foo 10 9 8))

"
            )
            .is_ok());
        assert!(smli::ProgramParser::new()
            .parse(
                r"(define bar (fun (x) (+ x 1)))

(define bar-z (fun () 2))

(print-num (bar (bar-z)))

"
            )
            .is_ok());
        // bonus
    }
}
