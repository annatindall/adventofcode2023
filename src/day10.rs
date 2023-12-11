use std::str::Lines;

pub fn day10(input_lines: &str) -> (String, String) {
    let lines = input_lines.lines();
    let answer1 = part1(lines);
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(lines: Lines<'_>) -> u32 {
    let field = Field::create(lines);

    let mut prev = field.find_start();
    let mut current = prev.clone();
    let mut length = 1;

    loop {
        // Find next pipe in loop
        for neighbours in &current.openings {
            if let Some(maybe) = field.get_pipe(neighbours.x, neighbours.y) {
                if maybe != prev && current.connects_to(&maybe) {
                    prev = current;
                    current = maybe;
                    length += 1;
                    break;
                }
            }
        }
        if current.symbol == 'S' {
            break;
        }
    }

    println!("Length of loop {}", length);
    (length as f64 / 2.0).floor() as u32
}

struct Field(Vec<Vec<char>>);

impl Field {
    fn create(lines: Lines<'_>) -> Self {
        let mut line_vec = Vec::new();
        for line in lines {
            line_vec.push(line.chars().collect())
        }
        Self(line_vec)
    }

    fn find_start(&self) -> Pipe {
        for (j, line) in self.0.iter().enumerate() {
            for (i, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return Pipe::new(i.try_into().unwrap(), j.try_into().unwrap(), 'S');
                }
            }
        }
        panic!("No start in this field");
    }

    fn get_pipe(&self, x: i32, y: i32) -> Option<Pipe> {
        if x < 0 || y < 0 {
            return None;
        }
        let i: usize = x.try_into().unwrap();
        let j: usize = y.try_into().unwrap();
        if let Some(row) = self.0.get(j) {
            row.get(i).map(|symbol| Pipe::new(x, y, *symbol))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Clone)]
struct Loop {
    ends: Vec<Pipe>,
    is_main: bool,
    length: u32,
}

#[derive(PartialEq, Clone, Debug)]
struct Pipe {
    loc: Point,
    openings: Vec<Point>,
    symbol: char,
}

impl Pipe {
    fn new(x: i32, y: i32, symbol: char) -> Self {
        let openings = match symbol {
            '-' => vec![Point::new(x - 1, y), Point::new(x + 1, y)],
            '|' => vec![Point::new(x, y - 1), Point::new(x, y + 1)],
            'L' => vec![Point::new(x, y - 1), Point::new(x + 1, y)],
            'F' => vec![Point::new(x + 1, y), Point::new(x, y + 1)],
            'J' => vec![Point::new(x, y - 1), Point::new(x - 1, y)],
            '7' => vec![Point::new(x - 1, y), Point::new(x, y + 1)],
            '.' => Vec::new(),
            'S' => vec![
                Point::new(x - 1, y),
                Point::new(x + 1, y),
                Point::new(x, y - 1),
                Point::new(x, y + 1),
            ],
            _ => panic!("Not a pipe"),
        };
        Self {
            loc: Point::new(x, y),
            openings,
            symbol,
        }
    }

    fn connects_to(&self, other: &Pipe) -> bool {
        other.openings.contains(&self.loc) && self.openings.contains(&other.loc)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        let field = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
        assert_eq!(part1(field.lines()), 8)
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(day10("").1, "0".to_string())
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(day10(""), ("0".to_string(), "0".to_string()))
    }
}
