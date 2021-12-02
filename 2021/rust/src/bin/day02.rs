use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let commands = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
    Ok(())
}

fn part1(commands: &[Command]) -> u64 {
    let mut sub = Submarine::default();
    sub.simple_navigate(commands);
    sub.depth * sub.position
}

fn part2(commands: &[Command]) -> u64 {
    let mut sub = Submarine::default();
    sub.navigate(commands);
    sub.depth * sub.position
}

#[derive(Debug, Clone)]
struct Submarine {
    position: u64,
    depth: u64,
    aim: u64,
}

impl Default for Submarine {
    fn default() -> Self {
        Submarine {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
}

impl Submarine {
    fn simple_navigate(&mut self, commands: &[Command]) {
        for &command in commands {
            // TODO: Check arithmetic
            match command {
                Command::Down(n) => self.depth += n,
                Command::Up(n) => {
                    if n > self.depth {
                        panic!("Submarine attempted to fly");
                    }
                    self.depth -= n;
                }
                Command::Forward(n) => self.position += n,
            }
        }
    }

    fn navigate(&mut self, commands: &[Command]) {
        for &command in commands {
            // TODO: Check arithmetic
            match command {
                Command::Down(n) => self.aim += n,
                Command::Up(n) => {
                    if n > self.aim {
                        panic!("Down is the only way forward!");
                    }
                    self.aim -= n;
                }
                Command::Forward(n) => {
                    self.position += n;
                    self.depth += self.aim * n;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl std::str::FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<_> = s.split(' ').collect();
        match parts[..] {
            ["forward", num] => Ok(Command::Forward(num.parse()?)),
            ["up", num] => Ok(Command::Up(num.parse()?)),
            ["down", num] => Ok(Command::Down(num.parse()?)),
            _ => Err(Box::new(AOCError::new("Failed to parse command"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, Submarine};

    static TEST_COMMANDS: &[Command] = &[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_simple_navigate() {
        let mut sub = Submarine::default();
        sub.simple_navigate(TEST_COMMANDS);
        assert_eq!(sub.position, 15);
        assert_eq!(sub.depth, 10);
    }

    #[test]
    fn test_navigate() {
        let mut sub = Submarine::default();
        sub.navigate(TEST_COMMANDS);
        assert_eq!(sub.position, 15);
        assert_eq!(sub.depth, 60);
    }
}
