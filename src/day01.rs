use std::collections::HashMap;

pub fn day01(input_lines: &str) -> (String, String) {
    let input = input_lines;
    let answer1 = part1(input);
    let answer2 = part2(input);

    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &str) -> u32 {
    input_lines
        .lines()
        .map(|l| {
            let calibration_val =
                first_digit_of(&mut l.chars()) + &first_digit_of(&mut l.chars().rev());
            calibration_val.parse::<u32>().unwrap()
        })
        .sum()
}

fn part2(input_lines: &str) -> u32 {
    input_lines
        .lines()
        .map(prune_trebuchet_line)
        .map(|chars| {
            let calibration_val = chars[0].to_string() + &chars[chars.len() - 1].to_string();
            calibration_val.parse::<u32>().unwrap()
        })
        .sum()
}

// Return only the numbers from the input
// Numbers are either ascii digits, or written out numbers one through nine
// All numbers in the output are written as ascii digits
fn prune_trebuchet_line(input: &str) -> Vec<char> {
    let mut numbers = HashMap::new();

    numbers.insert("one", '1');
    numbers.insert("two", '2');
    numbers.insert("three", '3');
    numbers.insert("four", '4');
    numbers.insert("five", '5');
    numbers.insert("six", '6');
    numbers.insert("seven", '7');
    numbers.insert("eight", '8');
    numbers.insert("nine", '9');

    let mut output = Vec::new();

    // Scan through with window-size 5 (max length of a number)
    // Add sufficient padding at the end for this window
    let input_string = input.to_string() + "xxxx";

    for chars_window in input_string.chars().collect::<Vec<_>>().windows(5) {
        if chars_window[0].is_ascii_digit() {
            output.push(chars_window[0])
        } else {
            // Check if this window starts with a written out number
            let window = chars_window.iter().collect::<String>();
            for english_number in numbers.keys() {
                if window.starts_with(english_number) {
                    output.push(numbers.get(*english_number).unwrap().to_owned())
                }
            }
            // Otherwise, window starts with an uninteresting char which we can discard
        }
    }

    output
}

fn first_digit_of<I>(input: &mut I) -> String
where
    I: Iterator<Item = char>,
{
    input
        .find(|c| c.is_ascii_digit())
        .expect("Input did not contain a digit")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        assert_eq!(part1(input), 142)
    }

    #[test]
    fn check_day01_part2_case1() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
        assert_eq!(part2(input), 281)
    }
}
