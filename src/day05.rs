use std::ops::Range;

pub fn day05(input_lines: &str) -> (String, String) {
    let mut lines = input_lines.lines();
    let seeds: Vec<i64> = lines.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    lines.next();
    let mappings = Mappings::create(lines);
    let answer1 = part1(&seeds, &mappings);
    let answer2 = part2(&seeds, &mappings);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(seeds: &[i64], mappings: &Mappings) -> i64 {
    seeds.iter().map(|seed| mappings.map(*seed)).min().unwrap()
}

fn part2(seed_numbers: &[i64], mappings: &Mappings) -> i64 {
    let mut ranges: Vec<Range<i64>> = Vec::new();

    for i in (0..seed_numbers.len()).step_by(2) {
        let start = seed_numbers[i];
        let len = seed_numbers[i + 1];
        ranges.push(start..(start + len));
    }

    let mut min_location = 0;
    loop {
        if min_location % 1000000 == 0 {
            println!("Trying location: {}", min_location);
        }
        let input = mappings.reverse_map(min_location);
        if ranges.iter().any(|range| range.contains(&input)) {
            break;
        }
        min_location += 1;
    }
    min_location
}

struct Mappings(Vec<Map>);

impl Mappings {
    fn create<'a, I>(input_lines: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut map_vec = Mappings(Vec::new());
        let mut current_map: Option<Map> = None;

        for line in input_lines {
            if let Some(mut map) = current_map {
                if line.is_empty() {
                    map_vec.0.push(map);
                    current_map = None;
                } else {
                    // Add a new entry to the current mapping
                    let mut numbers_split = line
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<i64>().unwrap());
                    map.entries.push(MapEntry {
                        dest_start: numbers_split.next().unwrap(),
                        source_start: numbers_split.next().unwrap(),
                        length: numbers_split.next().unwrap(),
                    });
                    current_map = Some(map);
                }
            } else {
                // Start a new mapping
                current_map = Some(Map::new());
            }
        }
        // EOF - end current map
        let map = current_map.unwrap();
        map_vec.0.push(map);
        map_vec
    }

    fn map(&self, source_value: i64) -> i64 {
        let mut current = source_value;
        for map in &self.0 {
            current = map.map(current)
        }
        current
    }

    fn reverse_map(&self, dest_value: i64) -> i64 {
        let mut current = dest_value;
        for map in self.0.iter().rev() {
            current = map.reverse_map(current)
        }
        current
    }
}

#[derive(Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn new() -> Self {
        Map {
            entries: Vec::new(),
        }
    }

    fn map(&self, input: i64) -> i64 {
        let correct_map = self.entries.iter().find(|map| map.has_mapping_for(input));
        match correct_map {
            Some(map) => map.map(input),
            None => input,
        }
    }

    fn reverse_map(&self, output: i64) -> i64 {
        let correct_map = self
            .entries
            .iter()
            .find(|map| map.has_reverse_mapping_for(output));
        match correct_map {
            Some(map) => map.reverse_map(output),
            None => output,
        }
    }
}

#[derive(Clone)]
struct MapEntry {
    source_start: i64,
    dest_start: i64,
    length: i64,
}

impl MapEntry {
    fn has_mapping_for(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }

    fn map(&self, input: i64) -> i64 {
        if self.has_mapping_for(input) {
            input + self.shift()
        } else {
            input
        }
    }

    fn shift(&self) -> i64 {
        self.dest_start - self.source_start
    }

    fn has_reverse_mapping_for(&self, dest: i64) -> bool {
        dest >= self.dest_start && dest < self.dest_start + self.length
    }

    fn reverse_map(&self, output: i64) -> i64 {
        if self.has_reverse_mapping_for(output) {
            output - self.shift()
        } else {
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_seeds_parsing() {
        let input = "seeds: 3429320627 235304036";
        let seeds: Vec<i64> = input[7..]
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        assert_eq!(seeds, vec![3429320627, 235304036])
    }

    #[test]
    fn check_single_map() {
        let mut map = Map::new();
        map.entries.push(MapEntry {
            dest_start: 50,
            source_start: 98,
            length: 2,
        });
        map.entries.push(MapEntry {
            dest_start: 52,
            source_start: 50,
            length: 48,
        });
        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(1), 1);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(51), 53);
        assert_eq!(map.map(99), 51);
    }
}
