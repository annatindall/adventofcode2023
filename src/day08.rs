use num::Integer;
use regex::Regex;
use std::collections::HashMap;
use std::str::{Chars, Lines};

pub fn day08(input_lines: &str) -> (String, String) {
    let mut lines = input_lines.lines();
    let directions = lines.next().unwrap().chars();
    lines.next().unwrap(); // Skip blank line
    let graph = DesertGraph::create(lines);

    let answer1 = part1(directions.clone(), &graph, "AAA");
    let answer2 = part2(directions, &graph);
    (format!("{}", answer1), format!("{:?}", answer2))
}

fn part1(directions: Chars<'_>, graph: &DesertGraph, start: &'static str) -> u64 {
    let mut steps = 0;
    let mut current = String::from(start);
    let end = "ZZZ";
    let directions = directions.cycle();

    for direction in directions {
        if current == end {
            break;
        }

        current = graph.get_hop(&current, direction);
        steps += 1;
    }
    steps
}

fn part2(directions: Chars<'_>, graph: &DesertGraph) -> u64 {
    // Let's make some ASSUMPTIONS:
    // For each 'A' node, you can follow directions to reach exactly one 'Z' node, after which a cycle repeats.
    // All the cycles start in exacly the same position - on the node after the initial 'A' node.
    // e.g.
    // A1Z1Z1Z1Z1Z1Z - cycle length 2
    // A12Z12Z12Z12Z - cycle length 3
    // A123Z123Z123Z - cycle length 4

    // With these assumptions, the position where all the 'Z' states line up is the LCM of the cycle lengths
    // For the example above, it's 12
    let start_states = graph
        .nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .cloned();
    let mut answer: u64 = 0;

    for start in start_states {
        let mut current = start;
        let mut steps = 0;
        for direction in directions.clone().cycle() {
            if current.ends_with('Z') {
                break;
            }

            current = graph.get_hop(&current, direction);
            steps += 1
        }
        if answer == 0 {
            answer = steps;
        } else {
            answer = answer.lcm(&steps);
        }
    }
    answer
}

struct DesertGraph {
    nodes: HashMap<String, Node>,
}

impl DesertGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn create(input_lines: Lines<'_>) -> Self {
        let mut graph = Self::new();
        let node_regex = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();

        for line in input_lines {
            for (_, [parent, left, right]) in node_regex.captures_iter(line).map(|c| c.extract()) {
                graph
                    .nodes
                    .entry(String::from(parent))
                    .or_insert_with(|| Node {
                        left: String::from(left),
                        right: String::from(right),
                    });
            }
        }

        graph
    }

    fn get_hop(&self, current_node: &str, direction: char) -> String {
        let current_node = self.nodes.get(current_node).unwrap();
        match direction {
            'L' => current_node.left.clone(),
            'R' => current_node.right.clone(),
            _ => panic!("That's not a direction"),
        }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part2() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let mut lines = input.lines();
        let directions = lines.next().unwrap().chars();
        lines.next().unwrap(); // Skip blank line
        let graph = DesertGraph::create(lines);
        assert_eq!(part2(directions, &graph), 6)
    }
}
