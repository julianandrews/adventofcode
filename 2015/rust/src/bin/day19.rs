#![feature(slice_split_once)]

use rustc_hash::{FxHashMap, FxHashSet};

use aoc::utils::get_input;

static RN: Atom = Atom(b'R' as u16 | (b'n' as u16) << 8);
static AR: Atom = Atom(b'A' as u16 | (b'r' as u16) << 8);
static Y: Atom = Atom(b'Y' as u16);

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let (rules, molecule) = parsing::parse_input(input.trim())?;

    println!("Part 1: {}", part1(&rules, &molecule));
    println!("Part 2: {}", part2(&rules, &molecule));

    Ok(())
}

fn part1(rules: &ReplacementRules, molecule: &Molecule) -> usize {
    rules.replacements(molecule).len()
}

fn part2(rules: &ReplacementRules, molecule: &Molecule) -> usize {
    rules.step_count(molecule)
}

#[derive(Debug, Clone)]
pub struct ReplacementRules {
    rules: FxHashMap<Atom, Vec<Molecule>>,
}

impl ReplacementRules {
    fn replacements(&self, molecule: &Molecule) -> FxHashSet<Molecule> {
        let mut outputs = FxHashSet::default();
        for (i, atom) in molecule.atoms.iter().enumerate() {
            if let Some(replacements) = self.rules.get(atom) {
                for replacement in replacements {
                    let mut atoms = molecule.atoms[..i].to_vec();
                    atoms.extend_from_slice(&replacement.atoms);
                    atoms.extend_from_slice(&molecule.atoms[i + 1..]);
                    outputs.insert(Molecule { atoms });
                }
            }
        }
        outputs
    }

    fn step_count(&self, molecule: &Molecule) -> usize {
        // The rules are all of the form A => BC where C is either an atom, or Rn...Ar where
        // '...' is a Y-separated non-empty list of other atoms. Rn, Ar, and Y never appear in
        // any other context.
        //
        // Each step therefore either:
        // - increases the atom count by one, or
        // - adds an Rn, an Ar, n Y atoms, and n + 1 other atoms.
        //
        // So ignoring all Rn and Ar atoms, each rule generates a single extra atom, plus twice
        // the Y count. The first rule gives us an extra atom from nothing, so we subtract 1.
        //
        // For those rules this formula *must* produce the correct step count, assuming a series
        // of substitutions exists.
        molecule.atoms.len().saturating_sub(1)
            - molecule.count(&RN)
            - molecule.count(&AR)
            - 2 * molecule.count(&Y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Molecule {
    atoms: Vec<Atom>,
}

impl Molecule {
    fn count(&self, atom: &Atom) -> usize {
        self.atoms.iter().filter(|&a| a == atom).count()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Atom(u16);

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 & ((1 << 8) - 1)) as u8 as char,
            (self.0 >> 8) as u8 as char
        )
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atom({})", self)
    }
}

mod parsing {
    use super::{Atom, Molecule, ReplacementRules};

    use anyhow::{anyhow, bail, Result};
    use rustc_hash::FxHashMap;

    impl std::str::FromStr for Molecule {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Molecule {
                atoms: parse_atoms(s)?,
            })
        }
    }

    impl std::str::FromStr for Atom {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match *s.as_bytes() {
                [a] => Ok(Atom(a as u16)),
                [a, b] => Ok(Atom(a as u16 | (b as u16) << 8)),
                _ => bail!("Invalid atom {}.", s),
            }
        }
    }

    impl std::str::FromStr for ReplacementRules {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut rules = FxHashMap::default();
            for line in s.lines() {
                let (from_part, to_part) = line
                    .split_once(" => ")
                    .ok_or_else(|| anyhow!("Invalid rule {}", s))?;
                let atom: Atom = from_part.parse()?;
                let molecule: Molecule = to_part.parse()?;
                rules.entry(atom).or_insert(vec![]).push(molecule);
            }
            Ok(ReplacementRules { rules })
        }
    }

    pub fn parse_input(s: &str) -> Result<(ReplacementRules, Molecule)> {
        let (rules_part, molecule_part) = s.split_once("\n\n").ok_or(anyhow!("Invalid input."))?;
        Ok((rules_part.parse()?, molecule_part.parse()?))
    }

    fn parse_atoms(s: &str) -> Result<Vec<Atom>> {
        let mut bytes = s.as_bytes();
        if bytes.iter().any(|b| !b.is_ascii_alphabetic()) {
            bail!("Invalid atoms {}.", s);
        }
        let mut atoms = vec![];
        while !bytes.is_empty() {
            let a = bytes[0] as u16;
            match bytes.get(1) {
                Some(b) if b.is_ascii_lowercase() => {
                    atoms.push(Atom(a | (*b as u16) << 8));
                    bytes = &bytes[2..];
                }
                _ => {
                    atoms.push(Atom(a));
                    bytes = &bytes[1..];
                }
            }
        }
        Ok(atoms)
    }
}

#[cfg(test)]
mod tests {
    use super::{parsing, Molecule};

    use rustc_hash::FxHashSet;

    static TEST_DATA: &str = "\
        H => HO\n\
        H => OH\n\
        O => HH\n\
        \n\
        HOH";

    #[test]
    fn replacements() {
        let (rules, molecule) = parsing::parse_input(TEST_DATA).unwrap();
        let result = rules.replacements(&molecule);
        let expected: FxHashSet<Molecule> = [
            "HOOH".parse().unwrap(),
            "HOHO".parse().unwrap(),
            "OHOH".parse().unwrap(),
            "HOOH".parse().unwrap(),
            "HHHH".parse().unwrap(),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }
}
