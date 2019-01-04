use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn get_sleep_counts(input: &str) -> Result<HashMap<usize, HashMap<usize, usize>>> {
    let mut sleep_counts = HashMap::new();
    let mut current_guard: usize = 0;
    let mut sleep_start: usize = 0;
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();

    for line in lines {
        let event_minute: usize = line[15..17].parse()?;
        if line.contains("begins shift") {
            current_guard = line[26..].split(" ").next().unwrap().parse()?;
        } else if line.contains("falls asleep") {
            sleep_start = event_minute;
        } else if line.contains("wakes up") {
            let counter = sleep_counts.entry(current_guard).or_insert(HashMap::new());
            for minute in sleep_start..event_minute {
                *counter.entry(minute).or_insert(0) += 1;
            }
        }
    }

    Ok(sleep_counts)
}

fn part1(input: &str) -> Result<()> {
    let sleep_counts = get_sleep_counts(input)?;

    let (sleepiest_guard, _) = sleep_counts
        .iter()
        .max_by_key(|(_, counter)| counter.values().sum::<usize>())
        .unwrap();

    let sleepiest_minute = sleep_counts
        .get(&sleepiest_guard)
        .unwrap()
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(&minute, _)| minute)
        .unwrap();

    writeln!(io::stdout(), "{}", sleepiest_guard * sleepiest_minute)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let sleep_counts = get_sleep_counts(input)?;

    let (sleepiest_guard, sleepiest_minute, _) = sleep_counts
        .iter()
        .flat_map(|(guard_id, counter)| {
            counter
                .iter()
                .map(move |(minute, count)| (guard_id, minute, count))
        }).max_by_key(|(_, _, &count)| count)
        .unwrap();

    writeln!(io::stdout(), "{}", sleepiest_guard * sleepiest_minute)?;
    Ok(())
}
