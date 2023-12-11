use std::collections::HashSet;

use itertools::Itertools;

pub fn day11(input_lines: &str) -> (String, String) {
    let picture = SpacePicture::create(input_lines);
    let answer1 = picture.sum_distances(2);
    let answer2 = picture.sum_distances(1000000);
    (format!("{}", answer1), format!("{}", answer2))
}

struct SpacePicture {
    galaxies: Vec<Point>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl SpacePicture {
    fn create(input_lines: &str) -> Self {
        let mut empty_rows: HashSet<usize> = (0..input_lines.lines().count()).collect();
        let mut empty_cols: HashSet<usize> =
            (0..input_lines.lines().next().unwrap().len()).collect();
        let mut galaxies = Vec::new();

        for (j, line) in input_lines.lines().enumerate() {
            for (i, symbol) in line.chars().enumerate() {
                if symbol == '#' {
                    empty_rows.remove(&j);
                    empty_cols.remove(&i);
                    galaxies.push(Point { x: i, y: j });
                }
            }
        }
        Self {
            galaxies,
            empty_rows: empty_rows.into_iter().collect(),
            empty_cols: empty_cols.into_iter().collect(),
        }
    }

    fn mapped_galaxies(&self, expansion_factor: usize) -> HashSet<Point> {
        self.galaxies
            .iter()
            .map(|galaxy| {
                let x_shift = self.empty_cols.iter().filter(|c| galaxy.x > **c).count()
                    * (expansion_factor - 1);
                let y_shift = self.empty_rows.iter().filter(|r| galaxy.y > **r).count()
                    * (expansion_factor - 1);
                Point {
                    x: galaxy.x + x_shift,
                    y: galaxy.y + y_shift,
                }
            })
            .collect()
    }

    fn sum_distances(&self, expansion_factor: usize) -> usize {
        let mapped_galaxies = self.mapped_galaxies(expansion_factor);
        mapped_galaxies
            .iter()
            .tuple_combinations()
            .map(|(g1, g2)| g1.l1_distance(g2))
            .sum()
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn l1_distance(&self, other: &Point) -> usize {
        usize::max(self.x, other.x) - usize::min(self.x, other.x) + usize::max(self.y, other.y)
            - usize::min(self.y, other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day11_part1_case1() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        let picture = SpacePicture::create(&input);
        assert_eq!(picture.sum_distances(2), 374)
    }

    #[test]
    fn check_day11_part2_case1() {
        let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        let picture = SpacePicture::create(&input);
        assert_eq!(picture.sum_distances(10), 1030)
    }
}
