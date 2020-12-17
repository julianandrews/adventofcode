#![feature(str_split_once)]

use std::collections::HashSet;
use std::ops::RangeInclusive;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let (fields, our_ticket, tickets) = parse_input(input.trim())?;

    println!("Part 1: {}", part1(&fields, &tickets));
    println!("Part 2: {}", part2(&fields, &our_ticket, &tickets)?);
    Ok(())
}

fn part1(fields: &[TicketField], tickets: &[Ticket]) -> usize {
    ticket_scanning_error_rate(fields, tickets)
}

fn part2(fields: &[TicketField], our_ticket: &Ticket, tickets: &[Ticket]) -> Result<usize> {
    let names = field_names(fields, tickets).ok_or(AOCError::new("Failed to identify fields"))?;
    Ok(names
        .iter()
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(i, _)| our_ticket.values[i])
        .product())
}

fn ticket_scanning_error_rate(fields: &[TicketField], tickets: &[Ticket]) -> usize {
    tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .values
                .iter()
                .filter(|&value| fields.iter().all(|field| !field.is_valid(value)))
        })
        .sum()
}

fn field_names<'a>(fields: &'a [TicketField], tickets: &[Ticket]) -> Option<Vec<&'a str>> {
    let valid_tickets: Vec<_> = tickets
        .iter()
        .filter(|ticket| {
            ticket
                .values
                .iter()
                .all(|value| fields.iter().any(|field| field.is_valid(value)))
                && ticket.values.len() == fields.len()
        })
        .collect();
    let mut field_options: Vec<HashSet<&str>> = (0..fields.len())
        .map(|i| {
            fields
                .iter()
                .filter(|field| {
                    valid_tickets
                        .iter()
                        .all(|ticket| field.is_valid(&ticket.values[i]))
                })
                .map(|field| field.name.as_str())
                .collect()
        })
        .collect();

    let mut solution = vec![None; fields.len()];
    while field_options.iter().any(|x| x.len() == 1) {
        for i in 0..field_options.len() {
            if field_options[i].len() == 1 {
                let value = field_options[i].iter().next().unwrap().clone();
                for j in 0..field_options.len() {
                    field_options.get_mut(j).unwrap().remove(value);
                }
                match solution[i] {
                    None => solution[i] = Some(value),
                    Some(_) => return None,
                }
            }
        }
    }
    solution.into_iter().collect()
}

#[derive(Debug)]
struct TicketField {
    name: String,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl TicketField {
    fn is_valid(&self, value: &usize) -> bool {
        self.ranges.0.contains(value) || self.ranges.1.contains(value)
    }
}

impl std::str::FromStr for TicketField {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (name, range_part) = s.split_once(": ").ok_or(AOCError::new("Invalid field"))?;
        let ranges = {
            let ranges: Vec<_> = range_part
                .split(" or ")
                .map(|s| {
                    let (a, b) = s.split_once('-').ok_or(AOCError::new("Invalid field"))?;
                    Ok(a.parse()?..=b.parse()?)
                })
                .collect::<Result<_>>()?;
            if ranges.len() != 2 {
                return Err(AOCError::new("Invalid field").into());
            }
            (ranges[0].clone(), ranges[1].clone())
        };
        Ok(TicketField {
            name: name.to_string(),
            ranges,
        })
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

impl std::str::FromStr for Ticket {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Ticket {
            values: parse_fields(s, ',')?,
        })
    }
}

fn parse_input(input: &str) -> Result<(Vec<TicketField>, Ticket, Vec<Ticket>)> {
    let parts: Vec<_> = input.split("\n\n").collect();
    if parts.len() != 3 {
        return Err(AOCError::new("Invalid input").into());
    }

    let fields = parse_fields(parts[0], '\n')?;

    let our_ticket = {
        let (header, ticket) = parts[1]
            .split_once('\n')
            .ok_or(AOCError::new("Invalid input"))?;
        if header != "your ticket:" {
            return Err(AOCError::new("Invalid input").into());
        }
        ticket.parse()?
    };

    let tickets = {
        let (header, tickets) = parts[2]
            .split_once('\n')
            .ok_or(AOCError::new("Invalid input"))?;
        if header != "nearby tickets:" {
            return Err(AOCError::new("Invalid input").into());
        }
        parse_fields(tickets, '\n')?
    };

    Ok((fields, our_ticket, tickets))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_rate() {
        let input = "class: 1-3 or 5-7\
                   \nrow: 6-11 or 33-44\
                   \nseat: 13-40 or 45-50\
                   \n\
                   \nyour ticket:\
                   \n7,1,14\
                   \n\
                   \nnearby tickets:\
                   \n7,3,47\
                   \n40,4,50\
                   \n55,2,20\
                   \n38,6,12";
        let (fields, _, tickets) = parse_input(input).unwrap();
        let result = ticket_scanning_error_rate(&fields, &tickets);
        assert_eq!(result, 71);
    }

    #[test]
    fn find_field_names() {
        let input = "class: 0-1 or 4-19\
                   \nrow: 0-5 or 8-19\
                   \nseat: 0-13 or 16-19\
                   \n\
                   \nyour ticket:\
                   \n11,12,13\
                   \n\
                   \nnearby tickets:\
                   \n3,9,18\
                   \n15,1,5\
                   \n5,14,9";
        let (fields, _, tickets) = parse_input(input).unwrap();
        let result = field_names(&fields, &tickets).unwrap();
        assert_eq!(result, vec!["row", "class", "seat"]);
    }
}
