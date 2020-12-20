extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

aoc_main!(parse_expenses);

fn parse_expenses(input: &str) -> Vec<u32> {
    let mut expenses = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    expenses.sort();
    expenses
}

fn part1(expenses: &Vec<u32>) {
    if let Some((lo, hi)) = find_pairwise_goal(&expenses, 2020, None) {
        println!("Found {}, {} = {}", lo, hi, lo * hi);
    } else {
        println!("Failed to find!");
    }
}

fn part2(expenses: &Vec<u32>) {
    // Subset-sum problem; this dataset is small enough to brute-force.
    for i in 0..expenses.len() {
        let goal = 2020 - expenses[i];
        if let Some((lo, hi)) = find_pairwise_goal(&expenses, goal, Some(i)) {
            println!(
                "Found {}, {}, {} = {}",
                lo,
                hi,
                expenses[i],
                lo * hi * expenses[i]
            );
            return;
        }
    }
    println!("Didn't find!");
}

fn find_pairwise_goal(
    expenses: &[u32],
    goal: u32,
    exclude_idx: Option<usize>,
) -> Option<(u32, u32)> {
    let mut idx_lo = 0;
    let mut idx_hi = expenses.len() - 1;

    while idx_lo < idx_hi {
        if exclude_idx.is_some() && idx_lo == exclude_idx.unwrap() {
            idx_lo += 1;
        }
        if exclude_idx.is_some() && idx_hi == exclude_idx.unwrap() {
            idx_hi -= 1;
        }

        let lo = expenses[idx_lo];
        let hi = expenses[idx_hi];
        let result = lo + hi;
        if result == goal {
            return Some((lo, hi));
        } else if result > goal {
            idx_hi -= 1;
            idx_lo = 0;
        } else {
            idx_lo += 1;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_pairwise_goal() {
        // Test case from https://adventofcode.com/2020/day/1 part 1.
        let expenses = parse_expenses(
            "1721
            979
            366
            299
            675
            1456",
        );
        let goal = find_pairwise_goal(&expenses, 2020, None);
        assert_eq!(goal, Some((299, 1721)));
    }
}
