pub fn day03(input_lines: &str) -> (String, String) {
    let schematic = Schematic::create(input_lines);
    let answer1 = schematic.sum_part_numbers();
    let answer2 = schematic.gear_ratio();
    (format!("{}", answer1), format!("{}", answer2))
}

struct Schematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn create(input_lines: &str) -> Self {
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();
        let mut number_builder = None;

        for (j, line) in input_lines.lines().enumerate() {
            for (i, token) in line.chars().enumerate() {
                if token.is_ascii_digit() {
                    let builder =
                        number_builder.get_or_insert(NumberBuilder::new(Point::new(i, j)));
                    builder.number_buf.push(token);
                } else {
                    // Any number has now ended
                    if let Some(builder) = number_builder {
                        numbers.push(builder.build(Point::new(i - 1, j)));
                        number_builder = None;
                    }
                    if token != '.' {
                        // Symbol
                        symbols.push(Symbol {
                            location: Point::new(i, j),
                        });
                    }
                }
            }
            // Numbers don't wrap over lines, so any number has now ended
            if let Some(builder) = number_builder {
                numbers.push(builder.build(Point::new(line.len(), j)));
                number_builder = None;
            }
        }
        Schematic { symbols, numbers }
    }

    fn sum_part_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| {
                self.symbols
                    .iter()
                    .any(|symbol| number.location.is_point_adjacent_to(&symbol.location))
            })
            .map(|number| number.value)
            .sum()
    }

    fn gear_ratio(&self) -> u32 {
        self.symbols
            .iter()
            .map(|symbol| {
                // Get adjacent numbers
                self.numbers
                    .iter()
                    .filter(|number| number.location.is_point_adjacent_to(&symbol.location))
                    .collect()
            })
            .filter(|adjacent_numbers: &Vec<&Number>| {
                adjacent_numbers.len() == 2 // Gears only
            })
            .map(|adjacent_numbers| {
                // Calculate gear ratio for this symbol
                adjacent_numbers
                    .iter()
                    .map(|num| num.value)
                    .product::<u32>()
            })
            .sum()
    }
}

struct Symbol {
    location: Point,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Point {
            x: i.try_into().unwrap(),
            y: j.try_into().unwrap(),
        }
    }
}

struct Number {
    value: u32,
    location: HorizontalLine,
}

struct NumberBuilder {
    left: Point,
    number_buf: String,
}

impl NumberBuilder {
    fn new(left: Point) -> Self {
        NumberBuilder {
            left,
            number_buf: String::new(),
        }
    }

    fn build(self, right: Point) -> Number {
        Number {
            value: self.number_buf.parse().unwrap(),
            location: HorizontalLine {
                left: self.left,
                right,
            },
        }
    }
}

struct HorizontalLine {
    left: Point,
    right: Point,
}

impl HorizontalLine {
    fn get_adjacent_area(&self) -> (Point, Point) {
        (
            Point {
                x: self.left.x - 1,
                y: self.left.y - 1,
            },
            Point {
                x: self.right.x + 1,
                y: self.right.y + 1,
            },
        )
    }

    fn is_point_adjacent_to(&self, point: &Point) -> bool {
        let (area_l, area_r) = self.get_adjacent_area();
        point.x >= area_l.x && point.x <= area_r.x && point.y >= area_l.y && point.y <= area_r.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        let input_lines = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let schematic = Schematic::create(input_lines);
        assert_eq!(schematic.sum_part_numbers(), 4361);
    }

    #[test]
    fn check_day03_part2_case1() {
        let input_lines = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let schematic = Schematic::create(input_lines);
        assert_eq!(schematic.gear_ratio(), 467835);
    }
}
