extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

type Forest = Vec<Vec<char>>;

aoc_main!(parse_forest);

fn parse_forest(input: &str) -> Forest {
    input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().chars().collect())
        .collect::<Forest>()
}

fn part1(forest: &Forest) {
    let tree_count = tree_count(forest, 3, 1);
    println!("trees: {}", tree_count);
}

fn part2(forest: &Forest) {
    let total = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(r, d)| tree_count(&forest, r, d))
        .fold(1u64, |acc, c| acc * c as u64);
    println!("total: {}", total);
}

fn tree_count(trees: &Forest, right: u32, down: u32) -> u32 {
    let mut y = 0;
    let mut x = 0;
    let mut ntrees = 0;
    while y < trees.len() {
        if trees[y][x] == '#' {
            ntrees += 1;
        }
        y += down as usize;
        x = (x + right as usize) % trees[0].len();
    }
    ntrees
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree_count() {
        // Test case from https://adventofcode.com/2020/day/3.
        let forest = parse_forest(
            r"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#",
        );
        assert_eq!(tree_count(&forest, 3, 1), 7);
    }
}
