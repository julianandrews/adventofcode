use aoc::aoc_error::AOCError;
use aoc::graphs::{toposort, Graph};
use std::collections::HashMap;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct Material {
    kind: String,
    quantity: usize,
}

impl Material {
    fn fuel(quantity: usize) -> Material {
        Material {
            kind: String::from("FUEL"),
            quantity: quantity,
        }
    }
}

impl FromStr for Material {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(AOCError::new("Invalid Material"))?;
        }

        Ok(Material {
            kind: parts[1].to_string(),
            quantity: parts[0].parse()?,
        })
    }
}

struct Reaction {
    inputs: Vec<Material>,
    output: Material,
}

impl FromStr for Reaction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, " => ").collect();
        if parts.len() != 2 {
            return Err(AOCError::new("Invalid Reaction"))?;
        }
        let inputs = parts[0]
            .split(", ")
            .map(&str::parse)
            .collect::<Result<Vec<Material>>>()?;
        let output = parts[1].parse()?;

        Ok(Reaction {
            inputs: inputs,
            output: output,
        })
    }
}

struct ReactionGraph {
    reactions: HashMap<String, Reaction>,
}

impl FromStr for ReactionGraph {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut reactions = HashMap::new();
        reactions.insert(
            "ORE".to_string(),
            Reaction {
                inputs: vec![],
                output: Material {
                    kind: "ORE".to_string(),
                    quantity: 1,
                },
            },
        );

        for line in s.lines() {
            let reaction: Reaction = line.trim().parse()?;
            if reactions.contains_key(&reaction.output.kind) {
                return Err(AOCError::new("Unexpected repeated output"))?;
            } else {
                reactions.insert(reaction.output.kind.clone(), reaction);
            }
        }

        Ok(ReactionGraph {
            reactions: reactions,
        })
    }
}

impl<'a> Graph<'a> for ReactionGraph {
    type Item = String;

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.reactions.keys().map(|s| s.to_string()))
    }

    fn neighbors(&'a self, value: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let neighbors = self
            .reactions
            .get(value)
            .map(|reaction| reaction.inputs.iter().map(|material| material.kind.clone()));

        match neighbors {
            Some(iter) => Box::new(iter),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl ReactionGraph {
    fn required_amounts(&self, goal: &Material) -> Result<HashMap<String, usize>> {
        let mut amounts: HashMap<String, usize> = HashMap::new();
        amounts.insert(goal.kind.clone(), goal.quantity);

        for kind in toposort(self).ok_or(AOCError::new("No toposort found for graph"))? {
            if let (Some(reaction), Some(needed)) = (self.reactions.get(&kind), amounts.get(&kind))
            {
                let multiple = aoc::nums::ceiling_div(*needed, reaction.output.quantity);
                for material in &reaction.inputs {
                    *amounts.entry(material.kind.clone()).or_insert(0) +=
                        material.quantity * multiple;
                }
            }
        }

        Ok(amounts)
    }

    fn required_ore(&self, goal: &Material) -> Result<usize> {
        Ok(self
            .required_amounts(goal)?
            .get("ORE")
            .ok_or(AOCError::new("ORE not found in raw inputs"))?
            .clone())
    }

    fn ore_fuel_yield(&self, available_ore: usize) -> Result<usize> {
        let mut min_fuel = 0; // Always less than or equal to the fuel yield
        let mut ceiling = None; // Always None or greater than the fuel yield

        while ceiling.is_none() || ceiling.unwrap() - min_fuel > 1 {
            let fuel = match ceiling {
                None => std::cmp::max(2 * min_fuel, 1),
                Some(value) => (min_fuel + value) / 2,
            };
            if self.required_ore(&Material::fuel(fuel))? > available_ore {
                ceiling = Some(fuel);
            } else {
                min_fuel = fuel;
            }
        }

        Ok(min_fuel)
    }
}

fn part1(reaction_graph: &ReactionGraph) -> Result<usize> {
    reaction_graph.required_ore(&Material::fuel(1))
}

fn part2(reaction_graph: &ReactionGraph) -> Result<usize> {
    reaction_graph.ore_fuel_yield(1000000000000)
}

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let reaction_graph: ReactionGraph = input.parse()?;

    println!("Part 1: {}", part1(&reaction_graph)?);
    println!("Part 2: {}", part2(&reaction_graph)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let reaction_graph = "9 ORE => 2 A
                              8 ORE => 3 B
                              7 ORE => 5 C
                              3 A, 4 B => 1 AB
                              5 B, 7 C => 1 BC
                              4 C, 1 A => 1 CA
                              2 AB, 3 BC, 4 CA => 1 FUEL"
            .parse::<ReactionGraph>()
            .unwrap();
        let result = reaction_graph.required_ore(&Material::fuel(1));
        assert_eq!(result.unwrap(), 165);
        let result = reaction_graph.ore_fuel_yield(100);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_example_2() {
        let reaction_graph = "157 ORE => 5 NZVS
                              165 ORE => 6 DCFZ
                              44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                              12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                              179 ORE => 7 PSHF
                              177 ORE => 5 HKGWZ
                              7 DCFZ, 7 PSHF => 2 XJWVT
                              165 ORE => 2 GPVTF
                              3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            .parse::<ReactionGraph>()
            .unwrap();
        let result = reaction_graph.required_ore(&Material::fuel(1));
        assert_eq!(result.unwrap(), 13312);
        let result = reaction_graph.ore_fuel_yield(1000000000000);
        assert_eq!(result.unwrap(), 82892753);
    }

    #[test]
    fn test_example_3() {
        let reaction_graph = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                              17 NVRVD, 3 JNWZP => 8 VPVL
                              53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                              22 VJHF, 37 MNCFX => 5 FWMGM
                              139 ORE => 4 NVRVD
                              144 ORE => 7 JNWZP
                              5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                              5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                              145 ORE => 6 MNCFX
                              1 NVRVD => 8 CXFTF
                              1 VJHF, 6 MNCFX => 4 RFSQX
                              176 ORE => 6 VJHF"
            .parse::<ReactionGraph>()
            .unwrap();
        let result = reaction_graph.required_ore(&Material::fuel(1));
        assert_eq!(result.unwrap(), 180697);
        let result = reaction_graph.ore_fuel_yield(1000000000000);
        assert_eq!(result.unwrap(), 5586022);
    }

    #[test]
    fn test_example_4() {
        let reaction_graph = "171 ORE => 8 CNZTR
                              7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                              114 ORE => 4 BHXH
                              14 VRPVC => 6 BMBT
                              6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                              6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                              15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                              13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                              5 BMBT => 4 WPTQ
                              189 ORE => 9 KTJDG
                              1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                              12 VRPVC, 27 CNZTR => 2 XDBXC
                              15 KTJDG, 12 BHXH => 5 XCVML
                              3 BHXH, 2 VRPVC => 7 MZWV
                              121 ORE => 7 VRPVC
                              7 XCVML => 6 RJRHP
                              5 BHXH, 4 VRPVC => 5 LTCX"
            .parse::<ReactionGraph>()
            .unwrap();
        let result = reaction_graph.required_ore(&Material::fuel(1));
        assert_eq!(result.unwrap(), 2210736);
        let result = reaction_graph.ore_fuel_yield(1000000000000);
        assert_eq!(result.unwrap(), 460664);
    }
}
