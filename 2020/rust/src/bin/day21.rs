#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let foods = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&foods));
    println!("Part 2: {}", part2(&foods)?);

    Ok(())
}

fn part1(foods: &[Food]) -> usize {
    let non_allergens = inert_ingredients(foods);

    foods
        .iter()
        .flat_map(Food::ingredients)
        .filter(|ingredient| non_allergens.contains(ingredient))
        .count()
}

fn part2(foods: &[Food]) -> Result<String> {
    let dangerous_ingredients: Vec<_> = {
        let mut allergens: Vec<_> = allergens(foods)
            .ok_or(AOCError::new("Failed to uniquely identify all allergens"))?
            .into_iter()
            .collect();
        allergens.sort_unstable();
        allergens
            .into_iter()
            .map(|(_allergen, ingredient)| ingredient)
            .collect()
    };

    Ok(dangerous_ingredients.join(","))
}

fn inert_ingredients(foods: &[Food]) -> HashSet<&str> {
    let potential_allergens = find_potential_allergens(foods)
        .into_iter()
        .flat_map(|(_allergen, ingredients)| ingredients.into_iter())
        .collect();
    let all_ingredients: HashSet<&str> = foods.iter().flat_map(Food::ingredients).collect();

    all_ingredients
        .difference(&potential_allergens)
        .cloned()
        .collect()
}

fn allergens(foods: &[Food]) -> Option<HashMap<&str, &str>> {
    let mut allergens = HashMap::new();
    let mut potential_allergens = find_potential_allergens(foods);
    while let Some((&allergen, &ingredient)) = potential_allergens
        .iter()
        .filter(|(_, ingredients)| ingredients.len() == 1)
        .map(|(allergen, ingredients)| (allergen, ingredients.iter().next().unwrap()))
        .next()
    {
        allergens.insert(allergen, ingredient);
        for ingredients in potential_allergens.values_mut() {
            ingredients.remove(ingredient);
        }
    }
    if allergens.len() == potential_allergens.len() {
        Some(allergens)
    } else {
        None
    }
}

fn find_potential_allergens(foods: &[Food]) -> HashMap<&str, HashSet<&str>> {
    let mut ingredients_by_allergen = HashMap::new();
    for food in foods.iter() {
        for allergen in food.allergens() {
            ingredients_by_allergen
                .entry(allergen)
                .or_insert_with(Vec::new)
                .push(food.ingredients().collect());
        }
    }
    ingredients_by_allergen
        .into_iter()
        .map(|(allergen, ingredient_lists)| (allergen, intersection(&ingredient_lists)))
        .collect()
}

fn intersection<T: Eq + std::hash::Hash + Clone>(sets: &[HashSet<T>]) -> HashSet<T> {
    sets[0]
        .iter()
        .filter(|value| sets.iter().all(|set| set.contains(value)))
        .cloned()
        .collect()
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn ingredients(&self) -> impl Iterator<Item = &str> {
        self.ingredients.iter().map(String::as_str)
    }

    fn allergens(&self) -> impl Iterator<Item = &str> {
        self.allergens.iter().map(String::as_str)
    }
}

impl std::str::FromStr for Food {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (ingredients_part, allergens_part) =
            s.split_once(" (").ok_or(AOCError::new("Invalid Food"))?;
        let ingredients = ingredients_part.split(' ').map(String::from).collect();
        let allergens = allergens_part
            .strip_prefix("contains ")
            .map(|s| s.strip_suffix(')'))
            .flatten()
            .ok_or(AOCError::new("Invalid allergen"))?
            .split(", ")
            .map(String::from)
            .collect();

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\
                                     \ntrh fvjkl sbzzf mxmxvkd (contains dairy)\
                                     \nsqjhc fvjkl (contains soy)\
                                     \nsqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn find_inert() {
        let foods = parse_fields(TEST_INPUT, '\n').unwrap();
        let expected = vec!["kfcds", "nhms", "sbzzf", "trh"].into_iter().collect();
        let result = inert_ingredients(&foods);
        assert_eq!(result, expected);
    }

    #[test]
    fn find_dangerous() {
        let foods = parse_fields(TEST_INPUT, '\n').unwrap();
        let expected = vec![("dairy", "mxmxvkd"), ("fish", "sqjhc"), ("soy", "fvjkl")]
            .into_iter()
            .collect();
        let result = allergens(&foods).unwrap();
        assert_eq!(result, expected);
    }
}
