use kaltui::{format_number, format_with_thousands, parse_and_eval, validate_parens};

#[test]
fn test_format_with_thousands() {
    assert_eq!(format_with_thousands(0), "0");
    assert_eq!(format_with_thousands(123), "123");
    assert_eq!(format_with_thousands(1234), "1'234");
    assert_eq!(format_with_thousands(1234567), "1'234'567");
    assert_eq!(format_with_thousands(-1234), "-1'234");
    assert_eq!(format_with_thousands(i64::MAX), "9'223'372'036'854'775'807");
}

#[test]
fn test_format_number() {
    assert_eq!(format_number(0.0), "0");
    assert_eq!(format_number(1000.0), "1'000");
    assert_eq!(format_number(1234.5), "1'234.5");
    assert_eq!(format_number(1000000.123), "1'000'000.123");
    assert_eq!(format_number(-1234.0), "-1'234");
}

#[test]
fn test_validate_parens() {
    assert!(validate_parens(""));
    assert!(validate_parens("()"));
    assert!(validate_parens("(())"));
    assert!(validate_parens("(()())"));
    assert!(validate_parens("2+(3*4)"));
    assert!(!validate_parens("("));
    assert!(!validate_parens(")"));
    assert!(!validate_parens(")("));
    assert!(!validate_parens("(()"));
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(parse_and_eval("2+3").unwrap(), 5.0);
    assert_eq!(parse_and_eval("10-3").unwrap(), 7.0);
    assert_eq!(parse_and_eval("4*5").unwrap(), 20.0);
    assert_eq!(parse_and_eval("20/4").unwrap(), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(parse_and_eval("2+3*4").unwrap(), 14.0);
    assert_eq!(parse_and_eval("10-2*3").unwrap(), 4.0);
    assert_eq!(parse_and_eval("20/4+1").unwrap(), 6.0);
}

#[test]
fn test_parentheses() {
    assert_eq!(parse_and_eval("(2+3)*4").unwrap(), 20.0);
    assert_eq!(parse_and_eval("2*(3+4)").unwrap(), 14.0);
    assert_eq!(parse_and_eval("((5))").unwrap(), 5.0);
}

#[test]
fn test_power() {
    assert_eq!(parse_and_eval("2^3").unwrap(), 8.0);
    assert_eq!(parse_and_eval("2^3^2").unwrap(), 512.0); // right-associative
}

#[test]
fn test_unary_minus() {
    assert_eq!(parse_and_eval("-5").unwrap(), -5.0);
    assert_eq!(parse_and_eval("-(3+2)").unwrap(), -5.0);
    assert_eq!(parse_and_eval("-(-5)").unwrap(), 5.0);
    assert_eq!(parse_and_eval("3+-2").unwrap(), 1.0);
}

#[test]
fn test_thousands_separator() {
    assert_eq!(parse_and_eval("1'000").unwrap(), 1000.0);
    assert_eq!(parse_and_eval("1'000'000+1").unwrap(), 1000001.0);
}

#[test]
fn test_errors() {
    assert!(parse_and_eval("").is_err());
    assert!(parse_and_eval("(").is_err());
    assert!(parse_and_eval("1/0").is_err());
    assert!(parse_and_eval("abc").is_err());
}
