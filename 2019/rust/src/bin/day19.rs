use aoc::intcode::{RegisterValue, VM};

fn main() -> anyhow::Result<()> {
    let input = aoc::utils::get_input()?;
    let tractor_beam = TractorBeam(aoc::intcode::parse_program(input.trim())?);

    println!("Part 1: {}", part1(&tractor_beam));
    println!("Part 2: {}", part2(&tractor_beam));

    Ok(())
}

fn part1(tractor_beam: &TractorBeam) -> usize {
    (0..50)
        .flat_map(|x| (0..50).map(move |y| tractor_beam.is_active(x, y)))
        .filter(|&b| b)
        .count()
}

fn part2(tractor_beam: &TractorBeam) -> RegisterValue {
    for (x, y) in tractor_beam.lower_left_edge() {
        if tractor_beam.is_active(x + 99, y - 99) {
            return 10000 * x + y - 99;
        }
    }
    unreachable!()
}

#[derive(Debug, Clone)]
struct TractorBeam(Vec<RegisterValue>);

impl TractorBeam {
    fn is_active(&self, x: RegisterValue, y: RegisterValue) -> bool {
        let mut vm = VM::new(self.0.clone(), Some(Box::new([x, y].into_iter())));
        let output = vm.outputs().next().unwrap_or(0);
        output != 0
    }

    fn lower_left_edge(&self) -> LowerLeftEdgeIterator {
        LowerLeftEdgeIterator {
            tractor_beam: &self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct LowerLeftEdgeIterator<'a> {
    tractor_beam: &'a TractorBeam,
    x: RegisterValue,
    y: RegisterValue,
}

impl<'a> Iterator for LowerLeftEdgeIterator<'a> {
    type Item = (RegisterValue, RegisterValue);

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.x, self.y);
        if self.tractor_beam.is_active(self.x, self.y + 1) {
            self.y += 1;
        } else if self.tractor_beam.is_active(self.x + 1, self.y) {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
            while !self.tractor_beam.is_active(self.x, self.y) {
                self.x += 1;
                if self.x > self.y {
                    self.x = 0;
                    self.y += 1;
                }
            }
        }
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use aoc::intcode::RegisterValue;

    use super::TractorBeam;

    fn get_program() -> Vec<RegisterValue> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("inputs/day19/input.txt");
        let input = std::fs::read_to_string(path).unwrap();
        aoc::intcode::parse_program(input.trim()).unwrap()
    }

    #[test]
    fn is_active() {
        let tractor_beam = TractorBeam(get_program());

        assert!(!tractor_beam.is_active(-1, 0));
        assert!(tractor_beam.is_active(0, 0));
        assert!(!tractor_beam.is_active(1, 0));
        assert!(tractor_beam.is_active(10, 14));
        assert!(!tractor_beam.is_active(9, 14));
    }

    #[test]
    fn lower_left_edge() {
        let tractor_beam = TractorBeam(get_program());

        let result: Vec<_> = tractor_beam.lower_left_edge().take(10).collect();
        let expected = [
            (0, 0),
            (3, 4),
            (4, 5),
            (5, 6),
            (5, 7),
            (6, 7),
            (6, 8),
            (7, 9),
            (8, 10),
            (8, 11),
        ];

        assert_eq!(result, expected);
    }
}
