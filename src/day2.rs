use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Report(Vec<usize>);

fn all_increasing(inp: &Vec<usize>) -> bool {
    inp.windows(2).all(|w| w[0] < w[1])
}

fn all_decreasing(inp: &Vec<usize>) -> bool {
    inp.windows(2).all(|w| w[0] > w[1])
}

fn all_acceptable_differences(inp: &Vec<usize>) -> bool {
    inp.windows(2).all(|w| {
        let difference = (w[0] as i64 - w[1] as i64).abs();
        difference < 4 && difference > 0
    })
}

impl Report {
    fn is_safe(&self) -> bool {
        (all_increasing(&self.0) || all_decreasing(&self.0)) && all_acceptable_differences(&self.0)
    }

    fn is_safe_part2(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        (0..self.0.len()).any(|i| {
            let sl = &[&self.0[..i], &self.0[i + 1..]].concat();
            Report(sl.to_vec()).is_safe()
        })
    }
}

impl std::str::FromStr for Report {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report(
            s.split_whitespace()
                .map(|d| d.parse().unwrap())
                .collect::<Vec<usize>>(),
        ))
    }
}

#[aoc_generator(day2)]
fn generate(input: &str) -> Vec<Report> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Report>) -> usize {
    input
        .iter()
        .filter_map(|r| r.is_safe().then(|| true))
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Report>) -> usize {
    input
        .iter()
        .filter_map(|r| r.is_safe_part2().then(|| true))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generate(EXAMPLE_INPUT)), 2)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generate(EXAMPLE_INPUT)), 4);
    }
}
