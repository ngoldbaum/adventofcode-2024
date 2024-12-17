use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

enum Op {
    Mul(usize, usize),
    Do,
    Dont,
}

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Op> {
    let mul_re = Regex::new(r"(mul\((\d*),(\d*)\))").unwrap();
    let mul_ops = mul_re
        .captures_iter(input)
        .map(|c| {
            let (_, [_, op1, op2]) = c.extract();
            (
                c.get(0).unwrap().start(),
                Op::Mul(op1.parse().unwrap(), op2.parse().unwrap()),
            )
        })
        .collect::<Vec<(usize, Op)>>();
    let do_re = Regex::new(r"(do\(\))").unwrap();
    let do_ops = do_re
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap().start(), Op::Do))
        .collect::<Vec<(usize, Op)>>();
    let dont_re = Regex::new(r"(don't\(\))").unwrap();
    let dont_ops = dont_re
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap().start(), Op::Dont))
        .collect::<Vec<(usize, Op)>>();
    let mut all_ops = mul_ops
        .into_iter()
        .chain(do_ops.into_iter().chain(dont_ops.into_iter()))
        .collect::<Vec<(usize, Op)>>();
    all_ops.sort_by(|v1, v2| v1.0.cmp(&v2.0));
    all_ops.into_iter().map(|v| v.1).collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Op>) -> usize {
    input.iter().fold(0, |acc, op| {
        acc + match op {
            Op::Mul(op1, op2) => op1 * op2,
            _ => 0,
        }
    })
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Op>) -> usize {
    let mut result = 0;
    let mut do_op = true;
    for op in input {
        match op {
            Op::Do => {
                do_op = true;
            }
            Op::Dont => {
                do_op = false;
            }
            Op::Mul(v1, v2) => {
                if do_op {
                    result += v1 * v2
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&generate(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&generate(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        )
    }
}
