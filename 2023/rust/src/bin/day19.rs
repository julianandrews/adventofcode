use std::ops::Range;

use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

use aoc::iterators::iter_pairs;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let (workflow_set, ratings) = parsing::parse_input(input.trim())?;

    println!("Part 1: {}", part1(&workflow_set, &ratings));
    println!("Part 2: {}", part2(&workflow_set)?);

    Ok(())
}

fn part1(workflow_set: &WorkflowSet, ratings: &[Rating]) -> u64 {
    ratings
        .iter()
        .filter(|rating| workflow_set.accepts(rating))
        .map(|rating| rating.x + rating.m + rating.a + rating.s)
        .sum()
}

fn part2(workflow_set: &WorkflowSet) -> Result<u64> {
    let accepted = workflow_set.accepted_ranges(RatingRange::default());

    // This step is slow and, on the real inputs, unecessary since no ranges intersect. But in
    // principle ranges could intersect (I think?), and then we'd need to find the true
    // intersection volume.
    if iter_pairs(&accepted).any(|(a, b)| !a.intersection(b).is_empty()) {
        bail!("Non empty intersection in ranges. Smarter algorithm required to find intersection volume");
    }

    Ok(accepted.iter().map(RatingRange::volume).sum())
}

#[derive(Debug, Clone)]
pub struct WorkflowSet<'a> {
    workflows: FxHashMap<&'a str, Workflow<'a>>,
}

impl<'a> WorkflowSet<'a> {
    fn accepts(&self, rating: &Rating) -> bool {
        let mut label = "in";
        while let Some(workflow) = self.workflows.get(label) {
            match workflow.run(rating) {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::GoTo(new_label) => label = new_label,
            }
        }
        false
    }

    fn accepted_ranges(&self, start: RatingRange) -> Vec<RatingRange> {
        let mut stack = vec![(start, Action::GoTo("in"))];
        let mut accepted = vec![];
        while let Some((range, action)) = stack.pop() {
            match action {
                Action::Accept => accepted.push(range),
                Action::Reject => {}
                Action::GoTo(label) => {
                    if let Some(workflow) = self.workflows.get(label) {
                        for (r, a) in workflow.run_range(&range) {
                            stack.push((range.intersection(&r), a));
                        }
                    }
                }
            }
        }
        accepted
    }
}

#[derive(Debug, Clone)]
pub struct Workflow<'a> {
    label: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn run(&self, rating: &Rating) -> &Action {
        for rule in &self.rules {
            match rule.condition {
                Some(condition) if !condition.test(rating) => {}
                _ => return &rule.action,
            }
        }
        unreachable!();
    }

    fn run_range(&self, range: &RatingRange) -> Vec<(RatingRange, Action)> {
        let mut output = vec![];
        let mut range = range.clone();
        for rule in &self.rules {
            match rule.condition {
                Some(condition) => {
                    let (passed_test, failed_test) = condition.test_range(&range);
                    output.push((passed_test, rule.action));
                    range = failed_test;
                }
                None => {
                    output.push((range, rule.action));
                    return output;
                }
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rule<'a> {
    condition: Option<Condition>,
    action: Action<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    GreaterThan(Category, u64),
    LessThan(Category, u64),
}

impl Condition {
    fn test(&self, rating: &Rating) -> bool {
        match *self {
            Condition::GreaterThan(category, value) => rating.get(category) > value,
            Condition::LessThan(category, value) => rating.get(category) < value,
        }
    }

    fn test_range(&self, range: &RatingRange) -> (RatingRange, RatingRange) {
        let (mut passed_test, mut failed_test) = (range.clone(), range.clone());
        match *self {
            Condition::GreaterThan(category, value) => {
                passed_test.get_mut(category).start = value + 1;
                failed_test.get_mut(category).end = value + 1;
            }
            Condition::LessThan(category, value) => {
                passed_test.get_mut(category).end = value;
                failed_test.get_mut(category).start = value;
            }
        }
        (passed_test, failed_test)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Rating {
    fn get(&self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatingRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl RatingRange {
    fn get_mut(&mut self, category: Category) -> &mut Range<u64> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn intersection(&self, other: &RatingRange) -> RatingRange {
        fn range_intersection(a: &Range<u64>, b: &Range<u64>) -> Range<u64> {
            a.start.max(b.start)..a.end.min(b.end)
        }

        RatingRange {
            x: range_intersection(&self.x, &other.x),
            m: range_intersection(&self.m, &other.m),
            a: range_intersection(&self.a, &other.a),
            s: range_intersection(&self.s, &other.s),
        }
    }

    fn volume(&self) -> u64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }
}

impl Default for RatingRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action<'a> {
    Accept,
    Reject,
    GoTo(&'a str),
}

mod parsing {
    use super::*;
    use std::str::FromStr;

    use anyhow::{anyhow, bail};
    use aoc::utils::parse_fields;

    pub fn parse_input(s: &str) -> Result<(WorkflowSet, Vec<Rating>)> {
        let (workflow_part, ratings_part) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Invalid input {}", s))?;
        let workflow = parse_workflowset(workflow_part)?;
        let ratings = parse_fields(ratings_part, '\n')?;

        Ok((workflow, ratings))
    }

    fn parse_workflowset(s: &str) -> Result<WorkflowSet> {
        let workflow_list: Vec<Workflow> = s.lines().map(parse_workflow).collect::<Result<_>>()?;
        let mut workflows = FxHashMap::default();
        for workflow in workflow_list {
            workflows.insert(workflow.label, workflow);
        }
        Ok(WorkflowSet { workflows })
    }

    fn parse_workflow(s: &str) -> Result<Workflow> {
        let (label, rule_part) = s
            .split_once('{')
            .ok_or_else(|| anyhow!("Invalid workflow {}", s))?;
        let rules: Vec<Rule> = rule_part
            .strip_suffix('}')
            .ok_or_else(|| anyhow!("Invalid workflow {}", s))?
            .split(',')
            .map(parse_rule)
            .collect::<Result<_>>()?;
        let last_rule = rules
            .last()
            .ok_or_else(|| anyhow!("No rules found in {}", s))?;
        if last_rule.condition.is_some() {
            bail!("Last rule in has condition in {}", s);
        }
        Ok(Workflow { label, rules })
    }

    fn parse_rule(s: &str) -> Result<Rule> {
        let (cond_part, action_part) = match s.split_once(':') {
            None => {
                return Ok(Rule {
                    condition: None,
                    action: parse_action(s)?,
                })
            }
            Some((cond_part, action_part)) => (cond_part, action_part),
        };
        let condition = Some(cond_part.parse()?);
        let action = parse_action(action_part)?;
        Ok(Rule { condition, action })
    }

    fn parse_action(s: &str) -> Result<Action> {
        match s {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Ok(Action::GoTo(s)),
            _ => Err(anyhow!("Invalid action {}", s)),
        }
    }

    impl FromStr for Category {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "x" => Ok(Category::X),
                "m" => Ok(Category::M),
                "a" => Ok(Category::A),
                "s" => Ok(Category::S),
                _ => Err(anyhow!("Unrecognized category {}", s)),
            }
        }
    }

    impl FromStr for Condition {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() < 3 || !s.is_ascii() {
                bail!("Invalid rule {}", s);
            }
            let (category_part, rest) = s.split_at(1);
            let (condition_part, value_part) = rest.split_at(1);
            let category = category_part.parse()?;
            let value = value_part.parse()?;
            match condition_part {
                ">" => Ok(Condition::GreaterThan(category, value)),
                "<" => Ok(Condition::LessThan(category, value)),
                _ => Err(anyhow!("Unrecognized condition {}", s)),
            }
        }
    }

    impl FromStr for Rating {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let inside = s
                .strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
                .ok_or_else(|| anyhow!("Invalid rating {}", s))?;
            let parts: FxHashMap<&str, &str> = inside
                .split(',')
                .map(|part| {
                    part.split_once('=')
                        .ok_or_else(|| anyhow!("Invalid rating part {} in {}", part, s))
                })
                .collect::<Result<_>>()?;
            let get_part = |var: &str| -> Result<u64> {
                let value = parts
                    .get(var)
                    .ok_or_else(|| anyhow!("{} missing in {}", var, s))?;
                Ok(value.parse()?)
            };
            Ok(Rating {
                x: get_part("x")?,
                m: get_part("m")?,
                a: get_part("a")?,
                s: get_part("s")?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}";

    fn build_range(x: Range<u64>, m: Range<u64>, a: Range<u64>, s: Range<u64>) -> RatingRange {
        RatingRange { x, m, a, s }
    }

    #[test]
    fn test_rating_1() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(workflow_set.accepts(&ratings[0]), true);
    }

    #[test]
    fn test_rating_2() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(workflow_set.accepts(&ratings[1]), false);
    }

    #[test]
    fn test_rating_3() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(workflow_set.accepts(&ratings[2]), true);
    }

    #[test]
    fn test_rating_4() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(workflow_set.accepts(&ratings[3]), false);
    }

    #[test]
    fn test_rating_5() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(workflow_set.accepts(&ratings[4]), true);
    }

    #[test]
    fn count_accepted() {
        let (workflow_set, ratings) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(part1(&workflow_set, &ratings), 19114);
    }

    #[test]
    fn run_range() {
        let (workflow_set, _) = parsing::parse_input(TEST_DATA).unwrap();
        let workflow = workflow_set.workflows.get("qqz").unwrap();
        #[rustfmt::skip]
        let expected = vec![
            (build_range(1..4001, 1..4001, 1..4001, 2771..4001), Action::GoTo("qs")),
            (build_range(1..4001, 1..1801, 1..4001, 1..2771), Action::GoTo("hdj")),
            (build_range(1..4001, 1801..4001, 1..4001, 1..2771), Action::Reject),
        ];
        assert_eq!(workflow.run_range(&RatingRange::default()), expected);
    }

    #[test]
    fn accepted_combinations() {
        let (workflow_set, _) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(part2(&workflow_set).unwrap(), 167409079868000);
    }
}
