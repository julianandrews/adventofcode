use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let expressions = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&expressions));
    println!("Part 2: {}", part2(&expressions));
    Ok(())
}

fn part1(expressions: &[Expression]) -> usize {
    expressions.iter().map(Expression::evaluate).sum()
}

fn part2(expressions: &[Expression]) -> usize {
    expressions.iter().map(Expression::really_evaluate).sum()
}

#[derive(Debug, Clone)]
struct Expression {
    operands: Vec<Operand>,
    operators: Vec<Operator>,
}

impl Expression {
    fn evaluate(&self) -> usize {
        let eval = |operand: &Operand| match operand {
            Operand::Value(x) => *x,
            Operand::Parenthetical(expression) => expression.evaluate(),
        };

        let mut result = eval(&self.operands[0]);
        for (operand, operator) in self.operands[1..].iter().zip(&self.operators) {
            result = match operator {
                Operator::Add => result + eval(operand),
                Operator::Multiply => result * eval(operand),
            };
        }
        result
    }

    fn really_evaluate(&self) -> usize {
        let eval = |operand: &Operand| match operand {
            Operand::Value(x) => *x,
            Operand::Parenthetical(expression) => expression.really_evaluate(),
        };

        let mut terms = vec![eval(&self.operands[0])];
        for (i, operator) in self.operators.iter().enumerate() {
            match operator {
                Operator::Multiply => terms.push(eval(&self.operands[i + 1])),
                Operator::Add => *terms.last_mut().unwrap() += eval(&self.operands[i + 1]),
            }
        }
        terms.iter().product()
    }
}

impl std::str::FromStr for Expression {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (mut s, operand) = trim_operand(s).ok_or(AOCError::new("Expected operand"))?;
        let mut operands = vec![operand];
        let mut operators = vec![];
        while !s.is_empty() {
            let pair = trim_operator(s).ok_or(AOCError::new("Expected operator"))?;
            s = pair.0;
            operators.push(pair.1);
            let pair = trim_operand(s).ok_or(AOCError::new("Expected operand"))?;
            s = pair.0;
            operands.push(pair.1);
        }
        Ok(Self {
            operands,
            operators,
        })
    }
}

fn trim_operand(s: &str) -> Option<(&str, Operand)> {
    if s.starts_with('(') {
        let i = find_closing_paren(s)?;
        let expression = s[1..i].parse().ok()?;
        Some((s[i + 1..].trim(), Operand::Parenthetical(expression)))
    } else {
        let i = s.find(' ').unwrap_or(s.len());
        let value = s[..i].parse().ok()?;
        Some((s[i..].trim(), Operand::Value(value)))
    }
}

fn trim_operator(s: &str) -> Option<(&str, Operator)> {
    match s.chars().next() {
        Some('+') => Some((s[1..].trim(), Operator::Add)),
        Some('*') => Some((s[1..].trim(), Operator::Multiply)),
        _ => None,
    }
}

fn find_closing_paren(s: &str) -> Option<usize> {
    let mut count = 1;
    for (i, c) in s[1..].char_indices() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => {}
        }
        if count == 0 {
            return Some(i + 1);
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Operand {
    Value(usize),
    Parenthetical(Expression),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case_1() {
        let expression: Expression = "1 + 2 * 3 + 4 * 5 + 6".parse().unwrap();
        assert_eq!(expression.evaluate(), 71);
    }

    #[test]
    fn simple_case_2() {
        let expression: Expression = "1 + (2 * 3) + (4 * (5 + 6))".parse().unwrap();
        assert_eq!(expression.evaluate(), 51);
    }

    #[test]
    fn simple_case_3() {
        let expression: Expression = "2 * 3 + (4 * 5)".parse().unwrap();
        assert_eq!(expression.evaluate(), 26);
    }

    #[test]
    fn simple_case_4() {
        let expression: Expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse().unwrap();
        assert_eq!(expression.evaluate(), 437);
    }

    #[test]
    fn simple_case_5() {
        let expression: Expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".parse().unwrap();
        assert_eq!(expression.evaluate(), 12240);
    }

    #[test]
    fn simple_case_6() {
        let expression: Expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .parse()
            .unwrap();
        assert_eq!(expression.evaluate(), 13632);
    }

    #[test]
    fn real_case_1() {
        let expression: Expression = "1 + 2 * 3 + 4 * 5 + 6".parse().unwrap();
        assert_eq!(expression.really_evaluate(), 231);
    }

    #[test]
    fn real_case_2() {
        let expression: Expression = "1 + (2 * 3) + (4 * (5 + 6))".parse().unwrap();
        assert_eq!(expression.really_evaluate(), 51);
    }

    #[test]
    fn real_case_3() {
        let expression: Expression = "2 * 3 + (4 * 5)".parse().unwrap();
        assert_eq!(expression.really_evaluate(), 46);
    }

    #[test]
    fn real_case_4() {
        let expression: Expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse().unwrap();
        assert_eq!(expression.really_evaluate(), 1445);
    }

    #[test]
    fn real_case_5() {
        let expression: Expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".parse().unwrap();
        assert_eq!(expression.really_evaluate(), 669060);
    }

    #[test]
    fn real_case_6() {
        let expression: Expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .parse()
            .unwrap();
        assert_eq!(expression.really_evaluate(), 23340);
    }
}
