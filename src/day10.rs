use std::str::Lines;

pub fn day10(input_lines: &str) -> (String, String) {
    let lines = input_lines.lines();
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(lines: Lines<'_>) -> u32 {
    let mut loops: Vec<Loop> = Vec::new();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pipe = Pipe::new(j.try_into().unwrap(), i.try_into().unwrap(), c);
            if let Some(l) = loops.iter().find(|l| l.can_connect(&pipe)) {
                l.connect(pipe);
            } else {
                println!("Adding pipe {} to new loop", c);
                loops.push(Loop::new(pipe));
            }
            // Merge loops
            for this_loop in loops {
                for other_loop in loops.iter().filter(|l| **l != this_loop) {

                }
            }
            // Remove any loops we've gone past

        }
    };

    println!("Loop length {}", loops.iter().find(|l| l.is_main).unwrap().length);
    0
}

#[derive(PartialEq, Clone)]
struct Loop {
    ends: Vec<Pipe>,
    is_main: bool,
    length: u32,
}

impl Loop {
    fn new(end: Pipe) -> Self {
        let is_main = end.symbol == 'S';
        Self {
            ends: vec![end],
            is_main,
            length: 0,
        }
    }
    fn can_connect(&self, pipe: &Pipe) -> bool {
        self.ends.iter().any(|end_pipe| end_pipe.connects_to(pipe))
    }

    fn connect(&mut self, pipe: Pipe) {
        // Assume guarded by can_connect()
        self.ends.retain(|end_pipe| !end_pipe.connects_to(&pipe));
        if pipe.symbol == 'S' {
            self.is_main = true;
        }
        if !self.ends.is_empty() {
            // Loop is not closed
            self.ends.push(pipe);
        }
        self.length += 1;
    }

    fn can_merge(&self, other: &Loop) -> bool {
        for this_end in self.ends.iter() {
            for other_end in other.ends.iter() {
                if this_end.connects_to(other_end) {
                    return true
                }
            }
        }
        false
    }
}

#[derive(PartialEq, Clone)]
struct Pipe {
    loc: Point,
    openings: Vec<Point>,
    symbol: char,
}

impl Pipe {
    fn new(x: i32, y: i32, symbol: char) -> Self {
        let openings = match symbol {
            '-' => vec![Point::new(x-1, y), Point::new(x+1, y)],
            '|' => vec![Point::new(x, y-1), Point::new(x, y+1)],
            'L' => vec![Point::new(x, y-1), Point::new(x+1, y)],
            'F' => vec![Point::new(x+1, y), Point::new(x, y+1)],
            'J' => vec![Point::new(x, y-1), Point::new(x-1, y)],
            '7' => vec![Point::new(x-1, y), Point::new(x, y+1)],
            '.' => Vec::new(),
            'S' => vec![Point::new(x-1, y), Point::new(x+1, y), Point::new(x, y-1), Point::new(x, y+1)],
            _ => panic!("Not a pipe")
        };
        Self {
            loc: Point::new(x,y),
            openings,
            symbol
        }
    }

    fn connects_to(&self, other: &Pipe) -> bool {
        println!("{} has loc: {:?} and openings: {:?}", self.symbol, self.loc, self.openings);
        println!("{} has loc: {:?} and openings: {:?}", other.symbol, other.loc, other.openings);
        let ans = other.openings.contains(&self.loc) && self.openings.contains(&other.loc);
        println!("Does it connect? {}", ans);
        ans
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
        assert_eq!(part1(field.lines()), 0)
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
