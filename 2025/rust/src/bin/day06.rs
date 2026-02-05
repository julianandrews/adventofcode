#![feature(iterator_try_collect)]

fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let problems = parsing::parse_input(&input)?;

    println!("{}", part1(&problems));
    println!("{}", part2(&problems));

    Ok(())
}

fn part1(problems: &[Problem]) -> u64 {
    problems.iter().map(|p| p.solve_wrong()).sum()
}

fn part2(problems: &[Problem]) -> u64 {
    problems.iter().map(|p| p.solve()).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    numbers: Vec<u64>,
    ceph_numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve_wrong(&self) -> u64 {
        self.compute(&self.numbers)
    }

    fn solve(&self) -> u64 {
        self.compute(&self.ceph_numbers)
    }

    fn compute(&self, numbers: &[u64]) -> u64 {
        match self.operation {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product(),
        }
    }
}

mod parsing {
    use anyhow::{anyhow, bail, Result};

    use crate::{Operation, Problem};

    pub fn parse_input(s: &str) -> Result<Vec<Problem>> {
        let line_width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        let num_rows = s.lines().count().saturating_sub(1);
        if s.lines().any(|line| line.len() != line_width) {
            bail!("Failed to parse input. Uneven lines deteceted.");
        }
        let columns = find_columns(s)?;

        let mut problems = vec![];
        for (start, end, operation) in columns {
            let mut numbers = vec![];
            let mut ceph_numbers = vec![0; end - start];
            for y in 0..num_rows {
                let row_offset = y * (line_width + 1);
                let slice = &s[row_offset..(row_offset + line_width)];
                numbers.push(slice[start..end].trim().parse()?);
                for x in start..end {
                    let b = slice.as_bytes()[x];
                    let i = x - start;
                    match b {
                        b'0'..=b'9' => ceph_numbers[i] = ceph_numbers[i] * 10 + (b - b'0') as u64,
                        b' ' => {}
                        _ => bail!("Unexpected character detected in '{}'", slice),
                    }
                }
            }
            problems.push(Problem {
                numbers,
                ceph_numbers,
                operation,
            });
        }
        Ok(problems)
    }

    fn find_columns(s: &str) -> Result<Vec<(usize, usize, Operation)>> {
        let line = s
            .lines()
            .last()
            .ok_or_else(|| anyhow::anyhow!("No operation line found"))?;
        let ops: Vec<(usize, Operation)> = line
            .char_indices()
            .filter_map(|(i, c)| match c {
                '*' => Some(Ok((i, Operation::Multiply))),
                '+' => Some(Ok((i, Operation::Add))),
                ' ' | '\t' => None,
                _ => Some(Err(anyhow!("Invalid character '{}' at position {}", c, i))),
            })
            .collect::<Result<_>>()?;
        Ok(ops
            .windows(2)
            .map(|window| {
                let &(start, op) = &window[0];
                let &(next_start, _) = &window[1];
                (start, next_start - 1, op)
            })
            .chain(ops.last().map(|&(start, op)| (start, line.len(), op)))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsing, Operation, Problem};

    static TEST_DATA: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    fn make_problem(numbers: Vec<u64>, ceph_numbers: Vec<u64>, operation: Operation) -> Problem {
        Problem {
            numbers,
            ceph_numbers,
            operation,
        }
    }

    #[test]
    fn parsing() {
        let problems = parsing::parse_input(TEST_DATA).unwrap();
        let expected = vec![
            make_problem(vec![123, 45, 6], vec![1, 24, 356], Operation::Multiply),
            make_problem(vec![328, 64, 98], vec![369, 248, 8], Operation::Add),
            make_problem(vec![51, 387, 215], vec![32, 581, 175], Operation::Multiply),
            make_problem(vec![64, 23, 314], vec![623, 431, 4], Operation::Add),
        ];

        assert_eq!(problems, expected);
    }

    #[test]
    fn solve_wrong() {
        let problems = parsing::parse_input(TEST_DATA).unwrap();
        let solutions: Vec<_> = problems.iter().map(|p| p.solve_wrong()).collect();
        assert_eq!(solutions, vec![33210, 490, 4243455, 401]);
    }

    #[test]
    fn solve() {
        let problems = parsing::parse_input(TEST_DATA).unwrap();
        let solutions: Vec<_> = problems.iter().map(|p| p.solve()).collect();
        assert_eq!(solutions, vec![8544, 625, 3253600, 1058]);
    }
}
