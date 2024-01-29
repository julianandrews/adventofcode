use anyhow::Result;
use aoc::intcode::{RegisterValue, VM};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let program: Vec<RegisterValue> = aoc::intcode::parse_program(input.trim())?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);

    Ok(())
}

fn part1(program: &[RegisterValue]) -> Result<RegisterValue> {
    // !(A & C) & D
    static SPRINGBOT_PROGRAM: &str = "\
        OR C J\n\
        AND A J\n\
        NOT J J\n\
        AND D J\n\
        WALK\n";
    Ok(survey_hull(program, SPRINGBOT_PROGRAM)?)
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    // !(A & B & C) & D & (E ^ H)
    static SPRINGBOT_PROGRAM: &str = "\
        NOT A J\n\
        NOT J J\n\
        AND B J\n\
        AND C J\n\
        NOT J J\n\
        AND D J\n\
        NOT E T\n\
        NOT T T\n\
        OR H T\n\
        AND T J\n\
        RUN\n";
    Ok(survey_hull(program, SPRINGBOT_PROGRAM)?)
}

fn survey_hull(
    program: &[RegisterValue],
    springbot_program: &str,
) -> Result<RegisterValue, SpringBotError> {
    let inputs = springbot_program.chars().map(|c| c as RegisterValue);
    let mut vm = VM::new(program.to_vec(), Some(Box::new(inputs)));
    let outputs: Vec<RegisterValue> = vm.outputs().collect();
    if let Some(&output) = outputs.last() {
        if output > 128 {
            return Ok(output);
        }
    }
    Err(SpringBotError(outputs))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpringBotError(Vec<RegisterValue>);

impl std::fmt::Display for SpringBotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &value in &self.0 {
            match u8::try_from(value) {
                Ok(b) => write!(f, "{}", b as char)?,
                Err(_) => {}
            }
        }
        Ok(())
    }
}

impl std::error::Error for SpringBotError {}

#[cfg(test)]
mod tests {
    use aoc::intcode::RegisterValue;

    use crate::SpringBotError;

    use super::survey_hull;

    static TEST_PROGRAM: &str = "\
        NOT D J\n\
        WALK\n";

    fn get_program() -> Vec<aoc::intcode::RegisterValue> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("inputs/day21/input.txt");
        let input = std::fs::read_to_string(path).unwrap();
        aoc::intcode::parse_program(input.trim()).unwrap()
    }

    #[test]
    fn failed_run() {
        let program = get_program();
        let result = survey_hull(&program, TEST_PROGRAM);
        let expected = "\
            Input instructions:\n\
            \n\
            Walking...\n\
            \n\
            \n\
            Didn't make it across:\n\
            \n\
            .................\n\
            .................\n\
            @................\n\
            #####.###########\n\
            \n\
            .................\n\
            .................\n\
            .@...............\n\
            #####.###########\n\
            \n\
            .................\n\
            ..@..............\n\
            .................\n\
            #####.###########\n\
            \n\
            ...@.............\n\
            .................\n\
            .................\n\
            #####.###########\n\
            \n\
            .................\n\
            ....@............\n\
            .................\n\
            #####.###########\n\
            \n\
            .................\n\
            .................\n\
            .....@...........\n\
            #####.###########\n\
            \n\
            .................\n\
            .................\n\
            .................\n\
            #####@###########\n\
            \n";
        let expected = SpringBotError(
            expected
                .chars()
                .map(|c| (c as u8) as RegisterValue)
                .collect(),
        );

        assert_eq!(result, Err(expected));
    }
}
