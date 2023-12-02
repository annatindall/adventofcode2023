use std::collections::HashMap;

pub fn day02(input_lines: &str) -> (String, String) {
    let answer1 = part1(input_lines);
    let answer2 = part2(input_lines);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &str) -> u32 {
    let true_bag = CubeSet::create_from_rgb(12, 13, 14);
    input_lines
        .lines()
        .map(Game::new)
        .filter(|game| game.is_compatible_with(&true_bag))
        .map(|game| game.id)
        .sum()
}

fn part2(input_lines: &str) -> u32 {
    input_lines
        .lines()
        .map(Game::new)
        .map(|game| {
            game.min_of_colour("red") * game.min_of_colour("green") * game.min_of_colour("blue")
        })
        .sum()
}
struct CubeSet {
    cubes_map: HashMap<String, u32>,
}

impl CubeSet {
    fn is_compatible_with(&self, true_bag: &CubeSet) -> bool {
        true_bag.total_cubes() >= self.total_cubes()
            && true_bag
                .cubes_map
                .iter()
                .all(|(colour, val)| val >= self.cubes_map.get(colour).unwrap_or(&0))
    }

    fn total_cubes(&self) -> u32 {
        self.cubes_map.values().sum()
    }

    fn create_from_rgb(red: u32, green: u32, blue: u32) -> Self {
        let mut obs = CubeSet {
            cubes_map: HashMap::new(),
        };
        obs.cubes_map.insert(String::from("red"), red);
        obs.cubes_map.insert(String::from("blue"), blue);
        obs.cubes_map.insert(String::from("green"), green);
        obs
    }
}

struct Game {
    id: u32,
    observations: Vec<CubeSet>,
}

impl Game {
    fn new(input_line: &str) -> Self {
        let mut initial_split = input_line.split(':');
        let id: u32 = initial_split.next().unwrap()[5..].parse().unwrap();
        let mut game = Game {
            id,
            observations: Vec::new(),
        };
        for obs in initial_split.next().unwrap().split(';') {
            // obs =  1 green, 2 red, 6 blue; 4 red, 1 green, 3 blue; 7 blue, 5 green; 6 blue, 2 red, 1 green
            let mut cubes_map = HashMap::new();
            for cube in obs.split(',').map(|s| s.trim()) {
                // cube = 1 green
                let mut split = cube.split(' ');
                let value = split.next().unwrap().parse().unwrap();
                let colour = split.next().unwrap();
                cubes_map.insert(colour.to_string(), value);
            }
            game.observations.push(CubeSet { cubes_map })
        }
        game
    }

    fn is_compatible_with(&self, true_bag: &CubeSet) -> bool {
        self.observations
            .iter()
            .all(|obs| obs.is_compatible_with(true_bag))
    }

    fn min_of_colour(&self, colour: &str) -> u32 {
        self.observations
            .iter()
            .map(|obs| *obs.cubes_map.get(colour).unwrap_or(&0))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        let game_line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::new(game_line);
        let true_bag = CubeSet::create_from_rgb(12, 13, 14);
        assert!(game.is_compatible_with(&true_bag));
    }

    #[test]
    fn check_day02_part1_case2() {
        let game_line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::new(game_line);
        let true_bag = CubeSet::create_from_rgb(12, 13, 14);
        assert!(!game.is_compatible_with(&true_bag));
    }

    #[test]
    fn check_day02_part2_case1() {
        let game_line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = Game::new(game_line);
        assert_eq!(game.min_of_colour("red"), 1);
        assert_eq!(game.min_of_colour("blue"), 4);
        assert_eq!(game.min_of_colour("green"), 3);
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(""), ("0".to_string(), "0".to_string()))
    }
}
