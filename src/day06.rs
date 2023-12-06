use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

pub fn day06(input_lines: &str) -> (String, String) {
    let part1_races = read_races_part1(input_lines);
    let answer1: u64 = part1_races
        .iter()
        .map(Race::count_record_breakers)
        .product();
    let part2_race = read_race_part2(input_lines);
    let answer2 = part2_race.count_record_breakers();
    (format!("{}", answer1), format!("{}", answer2))
}

fn read_races_part1(input_lines: &str) -> Vec<Race> {
    let mut lines = input_lines.lines();

    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<u64>().unwrap());

    times
        .zip(distances)
        .map(|(t, d)| Race {
            record_mm: d,
            time_ms: t,
        })
        .collect()
}

fn read_race_part2(input_lines: &str) -> Race {
    let mut lines = input_lines.lines();
    let time = parse_spaced_out_input_numbers(lines.next().unwrap());
    let distance = parse_spaced_out_input_numbers(lines.next().unwrap());
    Race {
        record_mm: distance,
        time_ms: time,
    }
}

fn parse_spaced_out_input_numbers(input_line: &str) -> u64 {
    input_line
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .fold(String::new(), |acc, s| acc + s.trim())
        .parse::<u64>()
        .unwrap()
}

struct Race {
    record_mm: u64,
    time_ms: u64,
}

impl Race {
    fn count_record_breakers(&self) -> u64 {
        // Solve quadratic inequality:
        // x^2 - rx + r < 0
        // x = button time
        // r = race time
        // d = record distance
        let r = BigDecimal::from_u64(self.time_ms).unwrap();
        let d = BigDecimal::from_u64(self.record_mm).unwrap();
        let discriminant: BigDecimal = &r * &r - 4 * &d;
        let upper_limit: f64 = (&r + discriminant.sqrt().unwrap()).to_f64().unwrap() / 2.0;
        let mut lower_limit: f64 = (&r - discriminant.sqrt().unwrap()).to_f64().unwrap() / 2.0;
        if lower_limit < 0.0 {
            lower_limit = 0.0;
        }
        (upper_limit.floor() - lower_limit.floor()) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let race = Race {
            record_mm: 9,
            time_ms: 7,
        };
        assert_eq!(race.count_record_breakers(), 4);
    }

    #[test]
    fn check_part2() {
        let race = Race {
            record_mm: 940200,
            time_ms: 71530,
        };
        assert_eq!(race.count_record_breakers(), 71503);
    }
}
