#![feature(str_split_once)]

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;
use std::collections::HashMap;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let passports: Vec<Passport> = input
        .trim()
        .split("\n\n")
        .map(&str::parse)
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&passports));
    println!("Part 2: {}", part2(&passports));
    Ok(())
}

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_really_valid()).count()
}

struct Passport {
    data: HashMap<String, String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.data.contains_key("byr")
            && self.data.contains_key("iyr")
            && self.data.contains_key("eyr")
            && self.data.contains_key("hgt")
            && self.data.contains_key("hcl")
            && self.data.contains_key("ecl")
            && self.data.contains_key("pid")
    }

    fn is_really_valid(&self) -> bool {
        self.is_year_in_range("byr", 1920..=2002)
            && self.is_year_in_range("iyr", 2010..=2020)
            && self.is_year_in_range("eyr", 2020..=2030)
            && self.height_valid()
            && self.hair_color_valid()
            && self.eye_color_valid()
            && self.passport_id_valid()
    }

    fn is_year_in_range(&self, field: &str, range: std::ops::RangeInclusive<usize>) -> bool {
        let year: Option<usize> = self
            .data
            .get(field)
            .map(|value| value.parse().ok())
            .flatten();
        match year {
            Some(year) => range.contains(&year),
            None => false,
        }
    }

    fn height_valid(&self) -> bool {
        match self.data.get("hgt") {
            Some(height) if height.len() > 2 => {
                let value: u64 = match &height[..height.len() - 2].parse() {
                    Ok(value) => *value,
                    Err(_) => return false,
                };
                match &height[height.len() - 2..] {
                    "cm" => (150..=193).contains(&value),
                    "in" => (59..=76).contains(&value),
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn hair_color_valid(&self) -> bool {
        match self.data.get("hcl") {
            Some(color) => {
                color.len() == 7
                    && color.chars().next() == Some('#')
                    && color[1..].chars().all(is_hex_digit)
            }
            None => false,
        }
    }

    fn eye_color_valid(&self) -> bool {
        match self.data.get("ecl") {
            Some(color) => match color.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            None => false,
        }
    }

    fn passport_id_valid(&self) -> bool {
        match self.data.get("pid") {
            Some(id) => id.len() == 9 && id.chars().all(|c| c.is_ascii_digit()),
            None => false,
        }
    }
}

impl FromStr for Passport {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .split_whitespace()
            .map(|s| {
                if let Some((k, v)) = s.split_once(':') {
                    Ok((k.to_string(), v.to_string()))
                } else {
                    Err(AOCError::new("Invalid passport field"))?
                }
            })
            .collect::<Result<_>>()?;

        Ok(Passport { data })
    }
}

fn is_hex_digit(c: char) -> bool {
    match c {
        ('0'..='9') | ('a'..='f') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let passport: Passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\
                                \nbyr:1937 iyr:2017 cid:147 hgt:183cm"
            .parse()
            .unwrap();
        assert!(passport.is_valid());
    }

    #[test]
    fn invalid_1() {
        let passport: Passport = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\
                                \nhcl:#cfa07d byr:1929"
            .parse()
            .unwrap();
        assert!(!passport.is_valid());
    }

    #[test]
    fn kind_of_valid() {
        let passport: Passport = "hcl:#ae17e1 iyr:2013\
                                \neyr:2024\
                                \necl:brn pid:760753108 byr:1931\
                                \nhgt:179cm"
            .parse()
            .unwrap();
        println!("{:?}", passport.data);
        assert!(passport.is_valid());
    }

    #[test]
    fn invalid_2() {
        let passport: Passport = "hcl:#cfa07d eyr:2025 pid:166559648\
                                \niyr:2011 ecl:brn hgt:59in"
            .parse()
            .unwrap();
        assert!(!passport.is_valid());
    }

    #[test]
    fn really_invalid() {
        let input = "eyr:1972 cid:100\
                   \nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\
                   \n\
                   \niyr:2019\
                   \nhcl:#602927 eyr:1967 hgt:170cm\
                   \necl:grn pid:012533040 byr:1946\
                   \n\
                   \nhcl:dab227 iyr:2012\
                   \necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\
                   \n\
                   \nhgt:59cm ecl:zzz\
                   \neyr:2038 hcl:74454a iyr:2023\
                   \npid:3556412378 byr:2007";
        for block in input.split("\n\n") {
            let passport: Passport = block.parse().unwrap();
            assert!(!passport.is_really_valid());
        }
    }

    #[test]
    fn really_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\
                   \nhcl:#623a2f\
                   \n\
                   \neyr:2029 ecl:blu cid:129 byr:1989\
                   \niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\
                   \n\
                   \nhcl:#888785\
                   \nhgt:164cm byr:2001 iyr:2015 cid:88\
                   \npid:545766238 ecl:hzl\
                   \neyr:2022\
                   \n\
                   \niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        for block in input.split("\n\n") {
            let passport: Passport = block.parse().unwrap();
            assert!(passport.is_really_valid());
        }
    }
}
