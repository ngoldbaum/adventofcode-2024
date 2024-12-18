use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn from_char(c: char) -> Result<Self, Box<dyn std::error::Error>> {
        match c {
            'X' => Ok(Letter::X),
            'M' => Ok(Letter::M),
            'A' => Ok(Letter::A),
            'S' => Ok(Letter::S),
            _ => Err("Parse error".into()),
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<Letter>>);

impl Map {
    fn get(&self, i: isize, j: isize) -> Option<Letter> {
        if i < 0 || j < 0 {
            return None;
        }
        let (i, j) = (i as usize, j as usize);
        self.0
            .get(j)
            .and_then(|line| line.get(i).and_then(|letter| Some(*letter)))
    }
    fn count_part1(&self) -> usize {
        let mut num_valid: usize = 0;
        let mut used: Vec<Vec<Option<Letter>>> = vec![vec![None; self.0[0].len()]; self.0.len()];
        for (j, i) in itertools::iproduct!(0..self.0.len(), 0..self.0[0].len()) {
            match self.get(i as isize, j as isize) {
                Some(Letter::X) => (),
                _ => continue,
            }
            let ilow = usize::saturating_sub(i, 3);
            let jlow = usize::saturating_sub(j, 3);
            let ranges: Vec<Vec<(usize, usize)>> = vec![
                (i + 1..i + 4).zip(std::iter::repeat(j)).collect(),
                (i + 1..i + 4).zip(j + 1..j + 4).collect(),
                std::iter::repeat(i).zip(j + 1..j + 4).collect(),
                ((ilow..i).rev()).zip(j + 1..j + 4).collect(),
                ((ilow..i).rev()).zip(std::iter::repeat(j)).collect(),
                ((ilow..i).rev()).zip((jlow..j).rev()).collect(),
                std::iter::repeat(i).zip((jlow..j).rev()).collect(),
                (i + 1..i + 4).zip((jlow..j).rev()).collect(),
            ];
            ranges
                .iter()
                .map(|range| {
                    (range
                        .iter()
                        .map(|(i, j)| self.get(*i as isize, *j as isize))
                        .collect::<Vec<Option<Letter>>>()
                        == vec![Some(Letter::M), Some(Letter::A), Some(Letter::S)])
                    .then(|| {
                        used[j][i] = Some(Letter::X);
                        used[range[0].1][range[0].0] = Some(Letter::M);
                        used[range[1].1][range[1].0] = Some(Letter::A);
                        used[range[2].1][range[2].0] = Some(Letter::S);
                        num_valid += 1
                    })
                })
                .count();
        }
        num_valid
    }

    fn count_part2(&self) -> usize {
        let mut num_valid: usize = 0;
        for (j, i) in itertools::iproduct!(0..self.0.len(), 0..self.0[0].len()) {
            match self.get(i as isize, j as isize) {
                Some(Letter::A) => (),
                _ => continue,
            }
            let ranges: Vec<(Vec<(isize, isize)>, Vec<(isize, isize)>)> = vec![
                (
                    vec![(-1, -1), (0, 0), (1, 1)],
                    vec![(-1, 1), (0, 0), (1, -1)],
                ),
                (
                    vec![(-1, -1), (0, 0), (1, 1)],
                    vec![(1, -1), (0, 0), (-1, 1)],
                ),
                (
                    vec![(1, -1), (0, 0), (-1, 1)],
                    vec![(1, 1), (0, 0), (-1, -1)],
                ),
                (
                    vec![(1, 1), (0, 0), (-1, -1)],
                    vec![(-1, 1), (0, 0), (1, -1)],
                ),
            ];
            ranges
                .iter()
                .map(|(range1, range2)| {
                    ((
                        range1
                            .iter()
                            .map(|(dx, dy)| self.get(i as isize + dx, j as isize + dy))
                            .collect::<Vec<Option<Letter>>>()
                            == vec![Some(Letter::M), Some(Letter::A), Some(Letter::S)],
                        range2
                            .iter()
                            .map(|(dx, dy)| self.get(i as isize + dx, j as isize + dy))
                            .collect::<Vec<Option<Letter>>>()
                            == vec![Some(Letter::M), Some(Letter::A), Some(Letter::S)],
                    ) == (true, true))
                        .then(|| num_valid += 1)
                })
                .count();
        }
        num_valid
    }
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Map {
    Map(input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Letter::from_char(c).unwrap())
                .collect()
        })
        .collect())
}

#[aoc(day4, part1)]
fn part1(input: &Map) -> usize {
    input.count_part1()
}

#[aoc(day4, part2)]
fn part2(input: &Map) -> usize {
    input.count_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generate(EXAMPLE_INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generate(EXAMPLE_INPUT)), 9)
    }
}
