pub fn day06(input_lines: &str) -> (String, String) {
    let races = read_races(input_lines);
    let answer1: u32 = races.iter().map(Race::count_record_breakers).product();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn read_races(input_lines: &str) -> Vec<Race> {
    let mut lines = input_lines.lines();

    let times = lines.next().unwrap().split_ascii_whitespace().skip(1).map(|t| t.parse::<u32>().unwrap());
    let distances = lines.next().unwrap().split_ascii_whitespace().skip(1).map(|t| t.parse::<u32>().unwrap());

    times.zip(distances).map(|(t,d)| {
        Race { record_mm: d, time_ms: t }
    }).collect()
}

struct Race {
    record_mm: u32,
    time_ms: u32
}

impl Race {
    fn count_record_breakers(&self) -> u32 {
        // Solve quadratic inequality:
        // x^2 - rx + d < 0
        // x = button time
        // r = race time
        // d = record distance
        let discriminant: f64 = (self.time_ms * self.time_ms - 4*self.record_mm).into();
        let upper_limit = (self.time_ms as f64 + discriminant.sqrt()) / 2.0;
        let mut lower_limit = (self.time_ms as f64 - discriminant.sqrt()) / 2.0;
        if lower_limit < 0.0 {
            lower_limit = 0.0;
        }
        (upper_limit.floor() - lower_limit.floor()) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_record_breakers() {
        let race = Race {
            record_mm: 9,
            time_ms: 7
        };
        assert_eq!(race.count_record_breakers(), 4);
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(day06("").1, "0".to_string())
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(day06(""), ("0".to_string(), "0".to_string()))
    }
}
