use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashMap;

use aoc::utils::get_input;

type MonkeyId = String;

fn main() -> Result<()> {
    let input = get_input()?;
    let mut monkey_riddle: MonkeyRiddle = input.trim().parse()?;

    println!("Part 1: {}", part1(&monkey_riddle)?);
    println!("Part 2: {}", part2(&mut monkey_riddle)?);

    Ok(())
}

fn part1(monkey_riddle: &MonkeyRiddle) -> Result<i64> {
    let expression = monkey_riddle.evaluate(&"root".to_owned())?;
    Ok(expression.a)
}

fn part2(monkey_riddle: &mut MonkeyRiddle) -> Result<i64> {
    monkey_riddle.fix();
    monkey_riddle.find_human_number()
}

#[derive(Debug, Clone)]
struct MonkeyRiddle {
    monkeys: FxHashMap<MonkeyId, Op>,
}

impl MonkeyRiddle {
    fn evaluate(&self, id: &MonkeyId) -> Result<Expression> {
        let op = self
            .monkeys
            .get(id)
            .ok_or_else(|| anyhow!("Missing monkey {}", id))?;
        match op {
            Op::Human => Ok(Expression { a: 0, b: 1, c: 1 }),
            Op::Number(n) => Ok(Expression { a: *n, b: 0, c: 1 }),
            Op::Add(a, b) => self.evaluate(a)?.add(self.evaluate(b)?),
            Op::Subtract(a, b) => self.evaluate(a)?.sub(self.evaluate(b)?),
            Op::Multiply(a, b) => self.evaluate(a)?.mul(self.evaluate(b)?),
            Op::Divide(a, b) => self.evaluate(a)?.div(self.evaluate(b)?),
        }
    }

    fn fix(&mut self) {
        self.monkeys.insert("humn".to_owned(), Op::Human);
    }

    fn find_human_number(&self) -> Result<i64> {
        let (lhs, rhs) = self
            .monkeys
            .get(&"root".to_owned())
            .and_then(|op| op.operands())
            .ok_or_else(|| anyhow!("Root operands not found"))?;
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;
        Ok((rhs.a * lhs.c - lhs.a * rhs.c) / (lhs.b * rhs.c - rhs.b * lhs.c))
    }
}

#[derive(Debug, Clone)]
enum Op {
    Human,
    Number(i64),
    Add(MonkeyId, MonkeyId),
    Subtract(MonkeyId, MonkeyId),
    Multiply(MonkeyId, MonkeyId),
    Divide(MonkeyId, MonkeyId),
}

impl Op {
    fn operands(&self) -> Option<(&MonkeyId, &MonkeyId)> {
        match self {
            Op::Add(lhs, rhs) => Some((lhs, rhs)),
            Op::Subtract(lhs, rhs) => Some((lhs, rhs)),
            Op::Multiply(lhs, rhs) => Some((lhs, rhs)),
            Op::Divide(lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
}

/// Expression encoding (a + bx) / c
#[derive(Debug, Clone, PartialEq)]
struct Expression {
    a: i64,
    b: i64,
    c: i64,
}

impl Expression {
    fn add(self, rhs: Self) -> Result<Self> {
        Ok(Self {
            a: self.a * rhs.c + rhs.a * self.c,
            b: self.b * rhs.c + rhs.b * self.c,
            c: self.c * rhs.c,
        }
        .reduce())
    }

    fn sub(self, rhs: Self) -> Result<Self> {
        Ok(Self {
            a: self.a * rhs.c - rhs.a * self.c,
            b: self.b * rhs.c - rhs.b * self.c,
            c: self.c * rhs.c,
        }
        .reduce())
    }

    fn mul(self, rhs: Self) -> Result<Self> {
        if self.b != 0 && rhs.b != 0 {
            bail!("Unexpected variable squaring");
        }
        Ok(Self {
            a: self.a * rhs.a,
            b: self.a * rhs.b + self.b * rhs.a,
            c: self.c * rhs.c,
        }
        .reduce())
    }

    fn div(self, rhs: Self) -> Result<Self> {
        if rhs.b != 0 {
            bail!("Unexpected division by variable");
        }
        Ok(Self {
            a: self.a,
            b: self.b,
            c: self.c * rhs.a,
        }
        .reduce())
    }

    fn reduce(&self) -> Self {
        let gcd = num::integer::gcd(self.a, self.b);
        let gcd = num::integer::gcd(gcd, self.c);
        Self {
            a: self.a / gcd,
            b: self.b / gcd,
            c: self.c / gcd,
        }
    }
}

impl std::str::FromStr for MonkeyRiddle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = FxHashMap::default();
        for line in s.split('\n') {
            let (id_part, op_part) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid line {}", line))?;
            monkeys.insert(id_part.to_owned(), op_part.parse()?);
        }

        Ok(Self { monkeys })
    }
}

impl std::str::FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            return Ok(Op::Number(n));
        }
        let (a, op, b) = s
            .split_once(' ')
            .and_then(|(a, rest)| {
                let (op, b) = rest.split_once(' ')?;
                Some((a, op, b))
            })
            .ok_or_else(|| anyhow!("Invalid op {}", s))?;
        let a = a.to_owned();
        let b = b.to_owned();
        match op {
            "+" => Ok(Op::Add(a, b)),
            "-" => Ok(Op::Subtract(a, b)),
            "*" => Ok(Op::Multiply(a, b)),
            "/" => Ok(Op::Divide(a, b)),
            _ => bail!("Invalid op {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32";

    #[test]
    fn evaluate_root() {
        let monkey_riddle: MonkeyRiddle = TEST_DATA.parse().unwrap();
        let result = monkey_riddle.evaluate(&"root".to_owned()).unwrap().a;

        assert_eq!(result, 152);
    }

    #[test]
    fn find_human_number() {
        let mut monkey_riddle: MonkeyRiddle = TEST_DATA.parse().unwrap();
        monkey_riddle.fix();
        let result = monkey_riddle.find_human_number().unwrap();

        assert_eq!(result, 301);
    }
}
