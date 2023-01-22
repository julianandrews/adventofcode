#![feature(generic_const_exprs)]

use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let players: Vec<Player> = parse_fields(input.trim(), '\n')?;
    let (player_1, player_2) = match &players[..] {
        &[a, b] => (a, b),
        _ => bail!("Expected exactly two players, found {}", players.len()),
    };

    println!("Part 1: {}", part1(player_1, player_2));
    println!("Part 2: {}", part2(player_1, player_2));

    Ok(())
}

fn part1(player_1: Player, player_2: Player) -> u64 {
    let mut players = [player_1, player_2];
    let mut dice = DeterministicDice(0);
    for i in 0.. {
        let player = &mut players[i % 2];
        *player = player.advance(dice.roll());
        if player.score >= 1000 {
            return players[(i + 1) % 2].score * (i as u64 + 1) * 3;
        }
    }
    unreachable!();
}

fn part2(player_1: Player, player_2: Player) -> u64 {
    let (p1_wins, p2_wins) = count_wins(player_1, player_2);
    p1_wins.max(p2_wins)
}

fn count_wins(player_1: Player, player_2: Player) -> (u64, u64) {
    let (p1_game_counts_by_round, p1_wins_by_round) = game_counts_and_wins::<21>(player_1);
    let (p2_game_counts_by_round, p2_wins_by_round) = game_counts_and_wins::<21>(player_2);

    // For each round where player 1 wins credit him wins for all player 2 games for that round.
    let p1_total_wins = p1_wins_by_round
        .iter()
        .zip(p2_game_counts_by_round)
        .fold(0, |total, (p1_wins, p2_games)| total + p1_wins * p2_games);

    // For each round where player 2 wins credit wins for all player 1 games for the next round.
    let p2_total_wins = p2_wins_by_round
        .iter()
        .zip(p1_game_counts_by_round.iter().skip(1))
        .fold(0, |total, (p2_wins, p1_games)| total + p2_wins * p1_games);

    (p1_total_wins, p2_total_wins)
}

/// Returns two arrays indexed by round of solo play.
///
/// The first array contains the number of games ongoing at the start of the round.
/// The second array contains the number of wins for the player in the round.
fn game_counts_and_wins<const N: usize>(player: Player) -> ([u64; N + 1], [u64; N + 1]) {
    const ROLL_COUNTS: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut live_games = [0; N + 1];
    let mut wins = [0; N + 1];

    let mut games = HashMap::new();
    games.insert(player, 1);
    let mut t = 0;

    while !games.is_empty() {
        t += 1;
        live_games[t] = games.values().sum();
        let mut new_games = HashMap::new();
        for (roll, roll_count) in ROLL_COUNTS {
            for (player, game_count) in &games {
                let new_player = player.advance(roll);
                if new_player.score >= N as u64 {
                    wins[t] += game_count * roll_count;
                } else {
                    *new_games.entry(new_player).or_insert(0) += game_count * roll_count;
                }
            }
        }
        games = new_games;
    }

    (live_games, wins)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    position: u64,
    score: u64,
}

impl Player {
    fn advance(&self, n: u64) -> Self {
        let position = (self.position + n - 1) % 10 + 1;
        let score = self.score + position;
        Self { position, score }
    }
}

struct DeterministicDice(u64);

impl DeterministicDice {
    fn roll(&mut self) -> u64 {
        let mut score = 0;
        for _ in 0..3 {
            self.0 = (self.0 + 1) % 100;
            score += self.0;
        }
        score
    }
}

impl std::str::FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, n) = s
            .split_once(" starting position: ")
            .ok_or_else(|| anyhow!("Failed to parse player {}", s))?;
        let position: u64 = n.parse()?;
        if position > 10 {
            bail!("Player off board!");
        }
        Ok(Self { position, score: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PLAYER_1: Player = Player {
        position: 4,
        score: 0,
    };
    static PLAYER_2: Player = Player {
        position: 8,
        score: 0,
    };

    #[test]
    fn deterministic_game() {
        let result = part1(PLAYER_1, PLAYER_2);
        assert_eq!(result, 739785);
    }

    #[test]
    fn counts_at_3() {
        let (game_counts, wins) = game_counts_and_wins::<3>(PLAYER_1);
        assert_eq!(wins, [0, 18, 243, 0]);
        assert_eq!(game_counts, [0, 1, 9, 0])
    }

    #[test]
    fn counts_at_4() {
        let (game_counts, wins) = game_counts_and_wins::<4>(PLAYER_1);
        assert_eq!(wins, [0, 17, 267, 81, 0]);
        assert_eq!(game_counts, [0, 1, 10, 3, 0]);
    }

    #[test]
    fn counts_at_5() {
        let (game_counts, wins) = game_counts_and_wins::<5>(PLAYER_1);
        assert_eq!(wins, [0, 17, 264, 162, 0, 0]);
        assert_eq!(game_counts, [0, 1, 10, 6, 0, 0]);
    }

    #[test]
    fn dirac_game() {
        let (p1_wins, p2_wins) = count_wins(PLAYER_1, PLAYER_2);
        assert_eq!(p1_wins, 444356092776315);
        assert_eq!(p2_wins, 341960390180808);
    }
}
