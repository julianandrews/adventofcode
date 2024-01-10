use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let instructions: Vec<Instruction> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));

    Ok(())
}

fn part1(instructions: &[Instruction]) -> u32 {
    let mut grid: Grid<ToggleGrid> = Grid::new();
    for instruction in instructions {
        grid.process(instruction);
    }
    grid.brightness()
}

fn part2(instructions: &[Instruction]) -> u32 {
    let mut grid: Grid<BrighnessGrid> = Grid::new();
    for instruction in instructions {
        grid.process(instruction);
    }
    grid.brightness()
}

struct ToggleGrid;
struct BrighnessGrid;

#[derive(Debug, Clone)]
struct Grid<T> {
    array: ndarray::Array2<u32>,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Grid<T> {
    fn new() -> Grid<T> {
        Grid {
            array: ndarray::Array2::zeros((1000, 1000)),
            phantom: std::marker::PhantomData,
        }
    }

    fn brightness(&self) -> u32 {
        self.array.sum()
    }
}

impl Grid<ToggleGrid> {
    fn process(&mut self, instruction: &Instruction) {
        let mut slice = self.array.slice_mut(instruction.slice_info);
        match instruction.action {
            Action::TurnOn => slice.fill(1),
            Action::TurnOff => slice.fill(0),
            Action::Toggle => slice.mapv_inplace(|x| (x != 1) as u32),
        }
    }
}

impl Grid<BrighnessGrid> {
    fn process(&mut self, instruction: &Instruction) {
        let mut slice = self.array.slice_mut(instruction.slice_info);
        match instruction.action {
            Action::TurnOn => slice.mapv_inplace(|x| x + 1),
            Action::TurnOff => slice.mapv_inplace(|x| x.saturating_sub(1)),
            Action::Toggle => slice.mapv_inplace(|x| x + 2),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    action: Action,
    slice_info: InstructionSliceInfo,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

type InstructionSliceInfo = ndarray::SliceInfo<
    [ndarray::SliceInfoElem; 2],
    ndarray::Dim<[usize; 2]>,
    ndarray::Dim<[usize; 2]>,
>;

mod parsing {
    use super::{Action, Instruction};

    use anyhow::{anyhow, Result};

    use aoc::iterators::AocIterators;

    impl std::str::FromStr for Instruction {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let [end, _, start, action_part] = s
                .rsplitn(4, ' ')
                .exactly_n::<4>()
                .ok_or_else(|| anyhow!("Invalid action {}.", s))?;
            let action = action_part.parse()?;
            let (x0, y0) = parse_point(start)?;
            let (x1, y1) = parse_point(end)?;
            Ok(Instruction {
                action,
                slice_info: ndarray::s![x0..x1 + 1, y0..y1 + 1],
            })
        }
    }

    impl std::str::FromStr for Action {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "turn on" => Ok(Action::TurnOn),
                "turn off" => Ok(Action::TurnOff),
                "toggle" => Ok(Action::Toggle),
                _ => Err(anyhow!("Unrecognized action {}.", s)),
            }
        }
    }

    fn parse_point(s: &str) -> Result<(i32, i32)> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid range {}.", s))?;
        Ok((a.parse()?, b.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{BrighnessGrid, Grid, ToggleGrid};

    #[test]
    fn process() {
        let mut grid: Grid<ToggleGrid> = Grid::new();
        grid.process(&"turn on 0,0 through 999,999".parse().unwrap());
        assert_eq!(grid.brightness(), 1000 * 1000);
        grid.process(&"toggle 0,0 through 999,0".parse().unwrap());
        assert_eq!(grid.brightness(), 1000 * 1000 - 1000);
        grid.process(&"turn off 499,499 through 500,500".parse().unwrap());
        assert_eq!(grid.brightness(), 1000 * 1000 - 1000 - 4);
    }

    #[test]
    fn process_brightness() {
        let mut grid: Grid<BrighnessGrid> = Grid::new();
        grid.process(&"turn on 0,0 through 0,0".parse().unwrap());
        assert_eq!(grid.brightness(), 1);
        grid.process(&"toggle 0,0 through 999,999".parse().unwrap());
        assert_eq!(grid.brightness(), 1 + 1000 * 1000 * 2);
    }
}
