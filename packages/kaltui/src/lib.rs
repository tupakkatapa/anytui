/// Validate that parentheses are balanced.
#[must_use]
pub fn validate_parens(expr: &str) -> bool {
    let mut depth = 0i32;
    for c in expr.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

/// Parse and evaluate a mathematical expression.
///
/// Supports: `+`, `-`, `*`, `/`, `^` (power), parentheses, unary minus.
/// Thousands separators (`'`) and spaces are stripped.
///
/// # Errors
/// Returns an error for invalid expressions, unmatched parentheses, or division by zero.
pub fn parse_and_eval(expr: &str) -> Result<f64, &'static str> {
    let expr = expr.replace([' ', '\''], "");
    if expr.is_empty() {
        return Err("Empty expression");
    }

    if !validate_parens(&expr) {
        return Err("Unmatched parentheses");
    }

    parse_expr(&expr)
}

fn parse_expr(expr: &str) -> Result<f64, &'static str> {
    let bytes = expr.as_bytes();

    // Handle addition and subtraction (lowest precedence, right-to-left scan)
    let mut depth: i32 = 0;
    for (i, &c) in bytes.iter().enumerate().rev() {
        match c {
            b')' => depth += 1,
            b'(' => depth = depth.saturating_sub(1),
            b'+' | b'-' if depth == 0 && i > 0 => {
                let prev = bytes[i - 1];
                if prev != b'+' && prev != b'-' && prev != b'*' && prev != b'/' && prev != b'(' {
                    if i + 1 >= expr.len() {
                        return Err("Invalid expression");
                    }
                    let left = parse_expr(&expr[..i])?;
                    let right = parse_expr(&expr[i + 1..])?;
                    return Ok(if c == b'+' {
                        left + right
                    } else {
                        left - right
                    });
                }
            }
            _ => {}
        }
    }

    // Handle multiplication and division
    depth = 0;
    for (i, &c) in bytes.iter().enumerate().rev() {
        match c {
            b')' => depth += 1,
            b'(' => depth = depth.saturating_sub(1),
            b'*' | b'/' if depth == 0 => {
                if i == 0 || i + 1 >= expr.len() {
                    return Err("Invalid expression");
                }
                let left = parse_expr(&expr[..i])?;
                let right = parse_expr(&expr[i + 1..])?;
                return Ok(if c == b'*' {
                    left * right
                } else {
                    if right.abs() < f64::EPSILON {
                        return Err("Division by zero");
                    }
                    left / right
                });
            }
            _ => {}
        }
    }

    // Handle exponentiation (right-to-left associativity, left-to-right scan)
    depth = 0;
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b'^' if depth == 0 => {
                if i == 0 || i + 1 >= expr.len() {
                    return Err("Invalid expression");
                }
                let left = parse_expr(&expr[..i])?;
                let right = parse_expr(&expr[i + 1..])?;
                let result = left.powf(right);
                if !result.is_finite() {
                    return Err("Invalid result");
                }
                return Ok(result);
            }
            _ => {}
        }
    }

    // Handle parentheses
    if expr.starts_with('(') && expr.ends_with(')') {
        return parse_expr(&expr[1..expr.len() - 1]);
    }

    // Handle unary minus
    if let Some(rest) = expr.strip_prefix('-') {
        return Ok(-parse_expr(rest)?);
    }

    // Parse number
    expr.parse::<f64>().map_err(|_| "Invalid number")
}

/// Format a number with thousands separators (apostrophe).
#[must_use]
pub fn format_number(n: f64) -> String {
    // Try to format as integer if it's a whole number
    if n.fract().abs() < f64::EPSILON
        && let Ok(int_val) = format!("{n:.0}").parse::<i64>()
    {
        return format_with_thousands(int_val);
    }

    // Fall back to decimal formatting
    let s = format!("{n:.10}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    // Format the integer part with thousands separator
    if let Some(dot_pos) = s.find('.') {
        let (int_part, dec_part) = s.split_at(dot_pos);
        if let Ok(i) = int_part.parse::<i64>() {
            format!("{}{}", format_with_thousands(i), dec_part)
        } else {
            s
        }
    } else {
        s
    }
}

/// Format an integer with thousands separators (apostrophe).
#[must_use]
pub fn format_with_thousands(n: i64) -> String {
    let (negative, abs_str) = if n == i64::MIN {
        (true, "9223372036854775808".to_string())
    } else {
        (n < 0, n.abs().to_string())
    };
    let chars: Vec<char> = abs_str.chars().collect();
    let mut result = String::new();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push('\'');
        }
        result.push(*c);
    }

    if negative {
        format!("-{result}")
    } else {
        result
    }
}
