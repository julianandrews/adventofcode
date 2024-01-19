use anyhow::{anyhow, Result};

use aoc::utils::{get_input, parse_fields};

type Volume = u64;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let buckets: Vec<Volume> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&buckets));
    println!("Part 2: {}", part2(&buckets)?);

    Ok(())
}

fn part1(buckets: &[Volume]) -> usize {
    partitions(150, buckets).len()
}

fn part2(buckets: &[Volume]) -> Result<usize> {
    let parts = partitions(150, buckets);
    let lengths: Vec<usize> = parts.iter().map(|p| p.len()).collect();
    let min_length = lengths.iter().min().ok_or(anyhow!("No buckets."))?;

    Ok(lengths.iter().filter(|&l| l == min_length).count())
}

fn partitions(total: Volume, buckets: &[Volume]) -> Vec<Vec<Volume>> {
    if total == 0 {
        return vec![vec![]];
    }
    let mut results = vec![];
    for (i, &bucket) in buckets.iter().enumerate() {
        if total >= bucket {
            for mut partition in partitions(total - bucket, &buckets[i + 1..]) {
                partition.push(bucket);
                results.push(partition);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::partitions;
    use aoc::testing::assert_matches_ignoring_order;

    #[test]
    fn all_partitions() {
        let result = partitions(25, &[20, 15, 10, 5, 5]);
        let expected = vec![vec![5, 20], vec![5, 20], vec![10, 15], vec![5, 5, 15]];

        assert_matches_ignoring_order(&result, &expected)
    }
}
