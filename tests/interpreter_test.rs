#[cfg(test)]
mod tests {
    use smli::grammer::ProgramParser;
    use smli::interpreter::Interpreter;

    #[test]
    fn test_case_1_1() {
        let parser = ProgramParser::new();
        assert!(parser.parse("(+)").is_err());
    }

    #[test]
    fn test_case_1_2() {
        let parser = ProgramParser::new();
        assert!(parser.parse("(+ (* 5 2) -)").is_err());
    }

    #[test]
    fn test_case_2_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num 1)
(print-num 2)
(print-num 3)
(print-num 4)

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"1
2
3
4"
            .to_string())
        );
    }

    #[test]
    fn test_case_2_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num 0)
(print-num -123)
(print-num 456)

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"0
-123
456"
            .to_string())
        );
    }

    #[test]
    fn test_case_3_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(+ 1 2 3)
(* 4 5 6)

(print-num (+ 1 (+ 2 3 4) (* 4 5 6) (/ 8 3) (mod 10 3)))

(print-num (mod 10 4))

(print-num (- (+ 1 2) 4))

(print-num -256)

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"133
2
-1
-256"
                .to_string())
        );
    }

    #[test]
    fn test_case_3_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num (mod 10 (+ 1 2)))

(print-num (* (/ 1 2) 4))

(print-num (- (+ 1 2 3 (- 4 5) 6 (/ 7 8) (mod 9 10))
              11))


",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"1
0
9"
            .to_string())
        );
    }

    #[test]
    fn test_case_4_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-bool #t)
(print-bool #f)

(print-bool (and #t #f))
(print-bool (and #t #t))

(print-bool (or #t #f))
(print-bool (or #f #f))

(print-bool (not #t))
(print-bool (not #f))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"#t
#f
#f
#t
#t
#f
#f
#t"
            .to_string())
        );
    }

    #[test]
    fn test_case_4_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-bool (or #t #t #f))
(print-bool (or #f (and #f #t) (not #f)))
(print-bool (and #t (not #f) (or #f #t) (and #t (not #t))))


",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"#t
#t
#f"
            .to_string())
        );
    }

    #[test]
    fn test_case_5_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num (if #t 1 2))

(print-num (if #f 1 2))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"1
2"
            .to_string())
        );
    }

    #[test]
    fn test_case_5_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num (if (< 1 2) (+ 1 2 3) (* 1 2 3 4 5)))

(print-num (if (= 9 (* 2 5))
               0
               (if #t 1 2)))",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"6
1"
            .to_string())
        );
    }

    #[test]
    fn test_case_6_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(define x 1)

(print-num x)

(define y (+ 1 2 3))

(print-num y)

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"1
6"
            .to_string())
        );
    }

    #[test]
    fn test_case_6_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(define a (* 1 2 3 4))

(define b (+ 10 -5 -2 -1))

(print-num (+ a b))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.run(ast_tree), Ok("26".to_string()));
    }

    #[test]
    fn test_case_7_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(print-num
  ((fun (x) (+ x 1)) 3))

(print-num
  ((fun (a b) (+ a b)) 4 5))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"4
9"
            .to_string())
        );
    }

    #[test]
    fn test_case_7_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(define x 0)

(print-num
  ((fun (x y z) (+ x (* y z))) 10 20 30))


(print-num x)

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.run(ast_tree),
            Ok(r"610
0"
            .to_string())
        );
    }

    #[test]
    fn test_case_8_1() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(define foo
  (fun (a b c) (+ a b (* b c))))

(print-num (foo 10 9 8))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.run(ast_tree), Ok("91".to_string()));
    }

    #[test]
    fn test_case_8_2() {
        let parser = ProgramParser::new();
        let ast_tree = parser
            .parse(
                r"(define bar (fun (x) (+ x 1)))

(define bar-z (fun () 2))

(print-num (bar (bar-z)))

",
            )
            .unwrap();
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.run(ast_tree), Ok("3".to_string()));
    }
}
