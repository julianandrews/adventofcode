#![feature(str_split_once)]

use std::collections::HashMap;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let (rules_part, message_part) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AOCError::new("Invalid input."))?;
    let messages: Vec<&str> = message_part.lines().collect();
    let mut ruleset: RuleSet = rules_part.parse()?;

    println!("Part 1: {}", part1(&mut ruleset, &messages));
    println!("Part 2: {}", part2(&mut ruleset, &messages));

    Ok(())
}

fn part1(ruleset: &mut RuleSet, messages: &[&str]) -> usize {
    messages
        .iter()
        .filter(|message| ruleset.matches(0, message))
        .count()
}

fn part2(ruleset: &mut RuleSet, messages: &[&str]) -> usize {
    ruleset.fix();

    messages
        .iter()
        .filter(|message| ruleset.matches(0, message))
        .count()
}

#[derive(Debug, Clone)]
struct RuleSet {
    rules: HashMap<usize, Rule>,
    memo: HashMap<usize, HashMap<String, bool>>,
}

impl RuleSet {
    fn matches(&mut self, id: usize, s: &str) -> bool {
        if self.memo.get(&id).map(|x| x.get(s)).flatten().is_none() {
            let result = self.matches_helper(id, s);
            self.memo
                .entry(id)
                .or_insert_with(HashMap::new)
                .insert(s.to_string(), result);
        }
        *self.memo.get(&id).unwrap().get(s).unwrap()
    }

    fn matches_helper(&mut self, id: usize, s: &str) -> bool {
        let rule = match self.rules.get(&id) {
            Some(rule) => rule.clone(),
            None => return false,
        };
        match rule {
            Rule::Terminal(c) => s.len() == 1 && s.chars().next().unwrap() == c,
            Rule::Simple(a) => self.matches_production_rule(&a, s),
            Rule::Choice(a, b) => {
                self.matches_production_rule(&a, s) || self.matches_production_rule(&b, s)
            }
        }
    }

    fn matches_production_rule(&mut self, production_rule: &ProductionRule, s: &str) -> bool {
        match production_rule {
            ProductionRule::Unit(a) => self.matches(*a, s),
            ProductionRule::Pair(a, b) => {
                (1..s.len()).any(|p| self.matches(*a, &s[..p]) && self.matches(*b, &s[p..]))
            }
        }
    }

    fn fix(&mut self) {
        self.rules.insert(
            8,
            Rule::Choice(ProductionRule::Unit(42), ProductionRule::Pair(42, 8)),
        );
        // Normalize "42 11 31" by adding a new rule.
        let new_rule_id = self.rules.keys().max().expect("We just inserted rule 8!") + 1;
        self.rules
            .insert(new_rule_id, Rule::Simple(ProductionRule::Pair(11, 31)));
        self.rules.insert(
            11,
            Rule::Choice(
                ProductionRule::Pair(42, 31),
                ProductionRule::Pair(42, new_rule_id),
            ),
        );
        self.memo.clear();
    }
}

impl std::str::FromStr for RuleSet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut rules = HashMap::new();
        for line in s.lines() {
            let (id, rule_part) = line.split_once(": ").ok_or(AOCError::new("Invalid rule"))?;
            rules.insert(id.parse()?, rule_part.parse()?);
        }

        Ok(RuleSet {
            rules,
            memo: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    Simple(ProductionRule),
    Choice(ProductionRule, ProductionRule),
}

impl std::str::FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s == "\"a\"" {
            Ok(Self::Terminal('a'))
        } else if s == "\"b\"" {
            Ok(Self::Terminal('b'))
        } else if s.contains('|') {
            let (a, b) = s.split_once(" | ").ok_or(AOCError::new("Invalid rule"))?;
            Ok(Self::Choice(a.parse()?, b.parse()?))
        } else {
            Ok(Self::Simple(s.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
enum ProductionRule {
    Unit(usize),
    Pair(usize, usize),
}

impl std::str::FromStr for ProductionRule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(' ') {
            let (a, b) = s.split_once(' ').ok_or(AOCError::new("Invalid rule"))?;
            Ok(Self::Pair(a.parse()?, b.parse()?))
        } else {
            Ok(Self::Unit(s.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruleset_1() {
        let input = "0: 1 2\
                   \n1: \"a\"\
                   \n2: 1 3 | 3 1\
                   \n3: \"b\"";
        let mut ruleset: RuleSet = input.parse().unwrap();

        assert_eq!(ruleset.matches(0, "aab"), true);
        assert_eq!(ruleset.matches(0, "aba"), true);
        assert_eq!(ruleset.matches(0, "ba"), false);
        assert_eq!(ruleset.matches(0, "abb"), false);
        assert_eq!(ruleset.matches(0, "aaa"), false);
        assert_eq!(ruleset.matches(0, "bbb"), false);
        assert_eq!(ruleset.matches(0, "aaba"), false);
    }

    #[test]
    fn ruleset_2() {
        // Normalized by ading rule 6.
        let input = "0: 4 6\
                   \n1: 2 3 | 3 2\
                   \n2: 4 4 | 5 5\
                   \n3: 4 5 | 5 4\
                   \n4: \"a\"\
                   \n5: \"b\"\
                   \n6: 1 5";
        let mut ruleset: RuleSet = input.parse().unwrap();

        assert_eq!(ruleset.matches(0, "ababbb"), true);
        assert_eq!(ruleset.matches(0, "bababa"), false);
        assert_eq!(ruleset.matches(0, "abbbab"), true);
        assert_eq!(ruleset.matches(0, "aaabbb"), false);
        assert_eq!(ruleset.matches(0, "aaaabbb"), false);
    }

    #[test]
    fn fixed_rules() {
        let input = "42: 9 14 | 10 1\
                   \n9: 14 27 | 1 26\
                   \n10: 23 14 | 28 1\
                   \n1: \"a\"\
                   \n11: 42 31\
                   \n5: 1 14 | 15 1\
                   \n19: 14 1 | 14 14\
                   \n12: 24 14 | 19 1\
                   \n16: 15 1 | 14 14\
                   \n31: 14 17 | 1 13\
                   \n6: 14 14 | 1 14\
                   \n2: 1 24 | 14 4\
                   \n0: 8 11\
                   \n13: 14 3 | 1 12\
                   \n15: 1 | 14\
                   \n17: 14 2 | 1 7\
                   \n23: 25 1 | 22 14\
                   \n28: 16 1\
                   \n4: 1 1\
                   \n20: 14 14 | 1 15\
                   \n3: 5 14 | 16 1\
                   \n27: 1 6 | 14 18\
                   \n14: \"b\"\
                   \n21: 14 1 | 1 14\
                   \n25: 1 1 | 1 14\
                   \n22: 14 14\
                   \n8: 42\
                   \n26: 14 22 | 1 20\
                   \n18: 15 15\
                   \n7: 14 5 | 1 21\
                   \n24: 14 1";
        let mut ruleset: RuleSet = input.parse().unwrap();

        // First with the original rules to establish a baseline
        assert_eq!(ruleset.matches(0, "bbabbbbaabaabba"), true);
        assert_eq!(ruleset.matches(0, "babbbbaabbbbbabbbbbbaabaaabaaa"), false);
        assert_eq!(
            ruleset.matches(0, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            false
        );
        assert_eq!(ruleset.matches(0, "bbbbbbbaaaabbbbaaabbabaaa"), false);
        assert_eq!(
            ruleset.matches(0, "bbbababbbbaaaaaaaabbababaaababaabab"),
            false
        );
        assert_eq!(ruleset.matches(0, "ababaaaaaabaaab"), true);
        assert_eq!(ruleset.matches(0, "ababaaaaabbbaba"), true);
        assert_eq!(ruleset.matches(0, "baabbaaaabbaaaababbaababb"), false);
        assert_eq!(ruleset.matches(0, "abbbbabbbbaaaababbbbbbaaaababb"), false);
        assert_eq!(ruleset.matches(0, "aaaaabbaabaaaaababaa"), false);
        assert_eq!(
            ruleset.matches(0, "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
            false
        );
        assert_eq!(
            ruleset.matches(0, "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
            false
        );

        // Now with fixed rules to verify that works.
        ruleset.fix();

        assert_eq!(ruleset.matches(0, "bbabbbbaabaabba"), true);
        assert_eq!(ruleset.matches(0, "babbbbaabbbbbabbbbbbaabaaabaaa"), true);
        assert_eq!(
            ruleset.matches(0, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            true
        );
        assert_eq!(ruleset.matches(0, "bbbbbbbaaaabbbbaaabbabaaa"), true);
        assert_eq!(
            ruleset.matches(0, "bbbababbbbaaaaaaaabbababaaababaabab"),
            true
        );
        assert_eq!(ruleset.matches(0, "ababaaaaaabaaab"), true);
        assert_eq!(ruleset.matches(0, "ababaaaaabbbaba"), true);
        assert_eq!(ruleset.matches(0, "baabbaaaabbaaaababbaababb"), true);
        assert_eq!(ruleset.matches(0, "abbbbabbbbaaaababbbbbbaaaababb"), true);
        assert_eq!(ruleset.matches(0, "aaaaabbaabaaaaababaa"), true);
        assert_eq!(
            ruleset.matches(0, "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
            true
        );
        assert_eq!(
            ruleset.matches(0, "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
            true
        );
    }
}
