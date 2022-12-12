use anyhow::{anyhow, bail, Result};

use aoc::utils::get_input;

type WorryLevel = u64;

fn main() -> Result<()> {
    let input = get_input()?;
    let monkey_circle: MonkeyCircle = input.parse()?;

    println!("Part 1: {}", part1(monkey_circle.clone()));
    println!("Part 2: {}", part2(monkey_circle));

    Ok(())
}

fn part1(mut monkey_circle: MonkeyCircle) -> usize {
    monkey_circle.monkey_business(20, true)
}

fn part2(mut monkey_circle: MonkeyCircle) -> usize {
    monkey_circle.monkey_business(10000, false)
}

#[derive(Debug, Clone)]
struct MonkeyCircle {
    monkeys: Vec<Monkey>,
    items: Vec<Vec<WorryLevel>>,
}

impl MonkeyCircle {
    fn monkey_business(&mut self, rounds: usize, relax: bool) -> usize {
        let modulo = self
            .monkeys
            .iter()
            .map(|monkey| monkey.test.divisible_by)
            .fold(1, num::integer::lcm);
        let n = self.monkeys.len();
        let mut items_handled = vec![0; n];
        for _ in 0..rounds {
            for (source, monkey) in self.monkeys.iter().enumerate() {
                items_handled[source] += self.items[source].len();
                for i in 0..self.items[source].len() {
                    let (destination, item) = monkey.handle(self.items[source][i], relax);
                    self.items[destination].push(item);
                }
                self.items[source].clear();
            }
            // Take the modulo to prevent worry overflow
            for monkey_items in self.items.iter_mut() {
                for item in monkey_items.iter_mut() {
                    *item %= modulo;
                }
            }
        }
        items_handled.sort_unstable();
        items_handled.get(n - 1).unwrap_or(&1) * items_handled.get(n - 2).unwrap_or(&1)
    }
}

impl std::str::FromStr for MonkeyCircle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = vec![];
        let mut items = vec![];
        for block in s.split("\n\n") {
            let (monkey, starting_items) = parse_block(block)?;
            monkeys.push(monkey);
            items.push(starting_items);
        }

        Ok(Self { monkeys, items })
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn handle(&self, item: WorryLevel, relax: bool) -> (usize, WorryLevel) {
        let value = match self.operation.value {
            OpValue::Number(x) => x,
            OpValue::Old => item,
        };
        let mut item = match self.operation.op {
            Op::Add => item + value,
            Op::Multiply => item * value,
        };
        if relax {
            item /= 3;
        }
        let destination = if item % self.test.divisible_by == 0 {
            self.test.if_target
        } else {
            self.test.else_target
        };
        (destination, item)
    }
}

#[derive(Debug, Clone)]
struct Operation {
    op: Op,
    value: OpValue,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Copy)]
enum OpValue {
    Number(WorryLevel),
    Old,
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible_by: WorryLevel,
    if_target: usize,
    else_target: usize,
}

fn parse_block(s: &str) -> Result<(Monkey, Vec<WorryLevel>)> {
    let lines: Vec<&str> = s.lines().collect();
    let items_str = strip_prefix_or_fail(lines[1], "  Starting items: ")?;
    let items = items_str
        .split(", ")
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let op_str = strip_prefix_or_fail(lines[2], "  Operation: new = old ")?;
    let op = match op_str.chars().next() {
        Some('+') => Op::Add,
        Some('*') => Op::Multiply,
        _ => bail!("Invalid operation: {}", op_str),
    };
    let (_, value_str) = op_str.split_at(2);
    let value = match value_str {
        "old" => OpValue::Old,
        _ => OpValue::Number(value_str.parse()?),
    };
    let divisible_by = strip_prefix_or_fail(lines[3], "  Test: divisible by ")?.parse()?;
    let if_target = strip_prefix_or_fail(lines[4], "    If true: throw to monkey ")?.parse()?;
    let else_target = strip_prefix_or_fail(lines[5], "    If false: throw to monkey ")?.parse()?;
    let monkey = Monkey {
        operation: Operation { op, value },
        test: Test {
            divisible_by,
            if_target,
            else_target,
        },
    };
    Ok((monkey, items))
}

fn strip_prefix_or_fail<'a>(s: &'a str, prefix: &str) -> Result<&'a str> {
    s.strip_prefix(prefix)
        .ok_or_else(|| anyhow!("Failed to strip '{}' from '{}'", prefix, s))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn monkey_business_1() {
        let mut monkey_circle: MonkeyCircle = TEST_DATA.parse().unwrap();
        let result = monkey_circle.monkey_business(20, true);
        assert_eq!(result, 10605);
    }

    #[test]
    fn monkey_business_2() {
        let mut monkey_circle: MonkeyCircle = TEST_DATA.parse().unwrap();
        let result = monkey_circle.monkey_business(10000, false);
        assert_eq!(result, 2713310158);
    }
}
