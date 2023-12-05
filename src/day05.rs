use std::collections::HashMap;

pub fn day05(input_lines: &str) -> (String, String) {
    let mut lines = input_lines.lines();
    let seeds: Vec<i64> = lines.next().unwrap()[7..].split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
    lines.next();
    let mappings = Mappings::create(lines);
    let answer1 = part1(&seeds, &mappings);
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(seeds: &Vec<i64>, mappings: &Mappings) -> i64 {
    seeds.iter().map(|seed| {
        mappings.map("seed", *seed, "location")
    }).min().unwrap()
}

struct Mappings(HashMap<String, Map>);

impl Mappings {
    fn create<'a, I>(mut input_lines: I) -> Self
    where I: Iterator<Item = &'a str>
    {
        let mut mappings = Mappings(HashMap::new());
        let mut current_map: Option<Map> = None;

        while let Some(line) = input_lines.next() {
           // Get first char of line
           if line == "" {
                // End the current mapping
                if let Some(map) = current_map { // There's some refactoring here
                    mappings.0.insert(map.source.to_owned(), map);
                    current_map = None;
                } else {
                    panic!("SOMETHING WENT WRONG");
                }
           } else if let Some(mut map) = current_map {
                // Add a new entry to the current mapping
                let mut numbers_split = line.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap());
                map.entries.push(MapEntry {
                    dest_start: numbers_split.next().unwrap(),
                    source_start: numbers_split.next().unwrap(),
                    length: numbers_split.next().unwrap()
                });
                current_map = Some(map);
           } else { // current_map.is_none()
                // Start a new mapping
                let mut map_name_split = line.split_ascii_whitespace().next().unwrap().split('-');
                let source = map_name_split.next().unwrap();
                let dest = map_name_split.nth(1).unwrap();
                current_map = Some(Map::new(source, dest));
           }
        }
        // EOF - end current map
        if let Some(map) = current_map { // There's some refactoring here
            mappings.0.insert(map.source.to_owned(), map);
        } else {
            panic!("SOMETHING WENT WRONG");
        }
        mappings
    }

    fn map(&self, source_name: &str, source_value: i64, dest_name: &str) -> i64 {
        let mut current_source = source_name;
        let mut current_value = source_value;
        while current_source != dest_name {
            let next_map = self.0.get(current_source).unwrap();
            current_value = next_map.map(current_value);
            current_source = &next_map.dest;
        }
        current_value
    }
}

struct Map {
    source: String,
    dest: String,
    entries: Vec<MapEntry>,
}

impl Map {
    fn new(source: &str, dest: &str) -> Self {
        Map {
            source: String::from(source),
            dest: String::from(dest),
            entries: Vec::new()
        }
    }

    fn map(&self, input: i64) -> i64 {
        let correct_map = self.entries.iter().find(|map| {
            map.has_mapping_for(input)
        });
        match correct_map {
            Some(map) => map.map(input),
            None => input
        }
    }
}


struct MapEntry {
    source_start: i64,
    dest_start: i64,
    length: i64
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_seeds_parsing() {
        let input = "seeds: 3429320627 235304036";
        let seeds: Vec<i64> = input[7..].split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
        assert_eq!(seeds, vec![3429320627, 235304036])
    }

    #[test]
    fn check_single_map() {
        let mut map = Map::new("seed", "soil");
        map.entries.push(MapEntry {
            dest_start: 50,
            source_start: 98,
            length: 2
        });
        map.entries.push(MapEntry {
            dest_start: 52,
            source_start: 50,
            length: 48
        });
        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(1), 1);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(51), 53);
        assert_eq!(map.map(99), 51);
    }
}