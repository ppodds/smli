#[macro_use]
extern crate lalrpop_util;

mod ast;
lalrpop_mod!(pub smli);

#[test]
fn test_num() {
    assert!(smli::ExpressionParser::new().parse("1").is_ok());
    assert!(smli::ExpressionParser::new().parse("0").is_ok());
    assert!(smli::ExpressionParser::new().parse("-1").is_ok());
    assert!(smli::ExpressionParser::new().parse("1.0").is_err());
}

#[test]
fn test_bool() {
    assert!(smli::ExpressionParser::new().parse("#t").is_ok());
    assert!(smli::ExpressionParser::new().parse("#f").is_ok());
}

#[test]
fn test_num_op() {
    assert!(smli::ExpressionParser::new().parse("(+ 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(+ 1 2 3 4)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(- 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(* 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(* 1 2 3 4)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(/ 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(mod 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(> 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(< 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(= 1 2)").is_ok());
    assert!(smli::ExpressionParser::new().parse("(= 1 2 3 4)").is_ok());
}

#[test]
fn test_logical_op() {
    assert!(smli::ExpressionParser::new().parse("(and #t #t)").is_ok());
    assert!(smli::ExpressionParser::new()
        .parse("(and #t #t #f #f)")
        .is_ok());
    assert!(smli::ExpressionParser::new().parse("(or #t #f)").is_ok());
    assert!(smli::ExpressionParser::new()
        .parse("(or #t #f #f #t)")
        .is_ok());
    assert!(smli::ExpressionParser::new().parse("(not #t)").is_ok());
}

#[test]
fn test_function_expression() {
    assert!(smli::ExpressionParser::new()
        .parse("(fun (a b) (+ a b))")
        .is_ok());
}

#[test]
fn test_function_call() {
    assert!(smli::ExpressionParser::new()
        .parse("((fun (x y z) (+ x y z)) 10 20 30)")
        .is_ok());
}

#[test]
fn test_define_statement() {
    assert!(smli::StatementParser::new().parse("(define a 1)").is_ok());
    assert!(smli::StatementParser::new().parse("(define b #t)").is_ok());
}

#[test]
fn test_expression_statement() {
    assert!(smli::StatementParser::new().parse("1").is_ok());
    assert!(smli::StatementParser::new().parse("#t").is_ok());
    assert!(smli::StatementParser::new().parse("(+ 1 2)").is_ok());
}

#[test]
fn test_print_statement() {
    assert!(smli::StatementParser::new().parse("(print-num 1)").is_ok());
    assert!(smli::StatementParser::new().parse("(print-num #t)").is_ok());
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

fn main() {
    println!("Hello, world!");
}
