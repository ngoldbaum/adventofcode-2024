use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Id(i64);

struct Input((Vec<Id>, Vec<Id>));

#[aoc_generator(day1)]
fn generate(input: &str) -> Input {
    Input(
        input
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|d| Id(d.parse().unwrap()))
                    .collect_tuple::<(Id, Id)>()
                    .unwrap()
            })
            .collect::<Vec<(Id, Id)>>()
            .into_iter()
            .unzip(),
    )
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> i64 {
    let (mut first_column, mut second_column) = (input.0 .0.clone(), input.0 .1.clone());
    first_column.sort();
    second_column.sort();
    first_column
        .iter()
        .zip(second_column.iter())
        .fold(0, |acc, d| acc + (d.0 .0 - d.1 .0).abs())
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> i64 {
    let (first_column, second_column) = (input.0 .0.clone(), input.0 .1.clone());
    first_column
        .iter()
        .fold(0, |acc, d| acc + d.0 * num_occurances(d.0, &second_column))
}

fn num_occurances(needle: i64, haystack: &Vec<Id>) -> i64 {
    haystack
        .iter()
        .fold(0, |acc, d| (acc + (d.0 == needle) as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generate(EXAMPLE_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generate(EXAMPLE_INPUT)), 31);
    }
}
