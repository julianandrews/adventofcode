#![feature(try_blocks)]

use aoc::combinatorics::permute;
use aoc::utils::{get_input, parse_fields};

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let ingredients: Vec<Ingredient> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&ingredients));
    println!("Part 2: {}", part2(&ingredients));

    Ok(())
}

fn part1(ingredients: &[Ingredient]) -> i64 {
    best_cookie_score(ingredients, |_cookie| true)
}

fn part2(ingredients: &[Ingredient]) -> i64 {
    best_cookie_score(ingredients, |cookie| cookie.calories() == 500)
}

fn best_cookie_score(ingredients: &[Ingredient], predicate: fn(&Cookie) -> bool) -> i64 {
    let mut best = i64::MIN;
    for partition in partitions(100, ingredients.len()) {
        let mut permutations = permute(partition);
        while let Some(parts) = permutations.next_perm() {
            let cookie = recipe(ingredients, parts);
            if predicate(&cookie) {
                best = best.max(cookie.total_score())
            }
        }
    }
    best
}

fn recipe(ingredients: &[Ingredient], parts: &[usize]) -> Cookie {
    let mut properties = [0; 5];
    for (ingredient, &parts) in ingredients.iter().zip(parts) {
        for (current, new) in properties.iter_mut().zip(ingredient.0) {
            *current += new * parts as i64;
        }
    }
    Cookie(properties)
}

#[derive(Debug, Clone)]
struct Ingredient([i64; 5]);

#[derive(Debug, Clone)]
struct Cookie([i64; 5]);

impl Cookie {
    fn total_score(&self) -> i64 {
        self.0.iter().take(4).map(|&p| p.max(0)).product()
    }

    fn calories(&self) -> i64 {
        self.0[4]
    }
}

fn partitions(n: usize, k: usize) -> PartitionIterator {
    if k < 2 || k > n {
        unimplemented!("I didn't bother to handle these cases.");
    }
    let mut values = vec![1; k];
    values[k - 1] = n + 2 - k;
    values[k - 2] = 0;
    PartitionIterator { values }
}

struct PartitionIterator {
    values: Vec<usize>,
}

impl Iterator for PartitionIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.values.len();
        let mut remaining = self.values[k - 1];
        let mut i = k - 2;
        while remaining == 1 {
            let value = &mut self.values[i];
            if *value > 1 {
                remaining += *value - 1;
                *value = 1;
            }
            if i == 0 {
                return None;
            }
            i -= 1;
        }
        remaining -= 1;
        self.values[i] += 1;
        self.values[k - 1] = remaining;
        Some(self.values.clone())
    }
}

mod parsing {
    use crate::Ingredient;

    impl std::str::FromStr for Ingredient {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Option<_> = try {
                let (_name, rest) = s.split_once(": capacity ")?;
                let (capacity, rest) = rest.split_once(", durability ")?;
                let (durability, rest) = rest.split_once(", flavor ")?;
                let (flavor, rest) = rest.split_once(", texture ")?;
                let (texture, calories) = rest.split_once(", calories ")?;
                (capacity, durability, flavor, texture, calories)
            };
            let (capacity, durability, flavor, texture, calories) =
                parts.ok_or_else(|| anyhow::anyhow!("Invalid ingredient {}.", s))?;
            Ok(Ingredient([
                capacity.parse()?,
                durability.parse()?,
                flavor.parse()?,
                texture.parse()?,
                calories.parse()?,
            ]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{recipe, Ingredient};

    static BUTTERSCOTCH: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
    static CINNAMON: &str = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn score() {
        let butterscotch: Ingredient = BUTTERSCOTCH.parse().unwrap();
        let cinnamon: Ingredient = CINNAMON.parse().unwrap();

        let cookie = recipe(&[butterscotch, cinnamon], &[44, 56]);
        assert_eq!(cookie.total_score(), 62842880);
    }

    #[test]
    fn score_and_calories() {
        let butterscotch: Ingredient = BUTTERSCOTCH.parse().unwrap();
        let cinnamon: Ingredient = CINNAMON.parse().unwrap();

        let cookie = recipe(&[butterscotch, cinnamon], &[40, 60]);
        assert_eq!(cookie.calories(), 500);
        assert_eq!(cookie.total_score(), 57600000);
    }
}
