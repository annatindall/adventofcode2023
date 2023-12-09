use itertools::rev;

pub fn day09(input_lines: &str) -> (String, String) {
    let inputs = input_lines.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
    });
    let answer1 = part1(inputs);
    let inputs_2 = input_lines.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
    });
    let answer2 = part2(inputs_2);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1<I, J>(sequences: I) -> i32
where
    I: Iterator<Item = J>,
    J: Iterator<Item = i32>,
{
    sequences.map(next_value).sum()
}

fn part2<I, J>(sequences: I) -> i32
where
    I: Iterator<Item = J>,
    J: DoubleEndedIterator<Item = i32>,
{
    sequences.map(rev).map(next_value).sum()
}

fn next_value<I>(input: I) -> i32
where
    I: Iterator<Item = i32>,
{
    let mut final_vals: Vec<i32> = Vec::new();
    let mut next_sequence: Vec<i32> = input.collect();
    while !next_sequence.iter().all(|n| *n == 0) {
        final_vals.push(*next_sequence.last().unwrap());
        next_sequence = next_sequence.windows(2).map(|s| s[1] - s[0]).collect();
    }
    final_vals.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1() {
        let input: Vec<i32> = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(next_value(input.into_iter()), 28)
    }
}
