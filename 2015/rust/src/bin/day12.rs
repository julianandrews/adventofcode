use anyhow::{anyhow, Result};
use serde_json::{Map, Value};

use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let data: Value = serde_json::from_str(input.trim())?;

    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

fn part1(data: &Value) -> Result<i64> {
    number_sum(data, |_| true)
}

fn part2(data: &Value) -> Result<i64> {
    number_sum(data, no_red)
}

type Predicate = fn(&Map<String, Value>) -> bool;

fn number_sum(data: &Value, predicate: Predicate) -> Result<i64> {
    fn item_sum<'a>(
        mut values: impl Iterator<Item = &'a Value>,
        predicate: Predicate,
    ) -> Result<i64> {
        values.try_fold(0, |total, data| Ok(total + number_sum(data, predicate)?))
    }

    match data {
        Value::Number(n) => n.as_i64().ok_or_else(|| anyhow!("Invalid number {}.", n)),
        Value::Array(values) => item_sum(values.iter(), predicate),
        Value::Object(m) if predicate(m) => item_sum(m.values(), predicate),
        _ => Ok(0),
    }
}

fn no_red(map: &Map<String, Value>) -> bool {
    !map.values()
        .any(|value| matches!(value, Value::String(s) if s == "red"))
}

#[cfg(test)]
mod tests {
    use super::{no_red, number_sum};

    use serde_json::Value;

    fn parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    #[test]
    fn number_sum_1() {
        assert_eq!(number_sum(&parse(r#"[1,2,3]"#), |_| true).unwrap(), 6);
    }

    #[test]
    fn number_sum_2() {
        assert_eq!(number_sum(&parse(r#"{"a":2,"b":4}"#), |_| true).unwrap(), 6);
    }

    #[test]
    fn number_sum_3() {
        assert_eq!(number_sum(&parse(r#"[[[3]]]"#), |_| true).unwrap(), 3);
    }

    #[test]
    fn number_sum_4() {
        assert_eq!(
            number_sum(&parse(r#"{"a":{"b":4},"c":-1}"#), |_| true).unwrap(),
            3
        );
    }

    #[test]
    fn number_sum_5() {
        assert_eq!(number_sum(&parse(r#"{"a":[-1,1]}"#), |_| true).unwrap(), 0);
    }

    #[test]
    fn number_sum_6() {
        assert_eq!(number_sum(&parse(r#"[-1,{"a":1}]"#), |_| true).unwrap(), 0);
    }

    #[test]
    fn number_sum_7() {
        assert_eq!(number_sum(&parse(r#"[]"#), |_| true).unwrap(), 0);
    }

    #[test]
    fn number_sum_8() {
        assert_eq!(number_sum(&parse(r#"{}"#), |_| true).unwrap(), 0);
    }

    #[test]
    fn no_red_sum_1() {
        assert_eq!(number_sum(&parse(r#"[1,2,3]"#), no_red).unwrap(), 6);
    }

    #[test]
    fn no_red_sum_2() {
        assert_eq!(
            number_sum(&parse(r#"[1,{"c":"red","b":2},3]"#), no_red).unwrap(),
            4
        );
    }

    #[test]
    fn no_red_sum_3() {
        assert_eq!(
            number_sum(&parse(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), no_red).unwrap(),
            0
        );
    }

    #[test]
    fn no_red_sum_4() {
        assert_eq!(number_sum(&parse(r#"[1,"red",5]"#), no_red).unwrap(), 6);
    }
}
