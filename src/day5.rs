extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

type BoardingPass<'a> = &'a str;

aoc_main!(parse_boarding_passes);

fn parse_boarding_passes(input: &str) -> Vec<&str> {
    input.split('\n').collect::<Vec<&str>>()
}

fn part1(boarding_passes: &Vec<&str>) {
    let ids = boarding_passes
        .iter()
        .filter_map(|b| boarding_pass_id(&b))
        .collect::<Vec<u16>>();
    let highest_id = ids.iter().max();
    println!("highest ID: {}", highest_id.expect("No valid IDs!"));
}

fn part2(boarding_passes: &Vec<&str>) {
    let mut ids = boarding_passes
        .iter()
        .filter_map(|b| boarding_pass_id(&b))
        .collect::<Vec<u16>>();
    ids.sort();

    for i in 0..ids.len() - 1 {
        let next_id = ids[i] + 1;
        if next_id != ids[i + 1] {
            println!("ID {} is missing", next_id);
            break;
        }
    }
}

fn boarding_pass_id(boarding_pass: BoardingPass) -> Option<u16> {
    // A boarding pass is just a binary number where 'B' and 'R' correspond to 1,
    // and 'F' and 'L' correspond to 0. We don't need to bother separately calculating
    // rows and columns as the calculating the ID simply shifts the bits to their
    // original locations in the binary number anyway.
    if boarding_pass.len() != 10 {
        None
    } else {
        boarding_pass.chars().try_fold(0, |acc, c| match c {
            'F' | 'L' => Some(acc * 2),
            'B' | 'R' => Some(acc * 2 + 1),
            _ => None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_boarding_pass_id() {
        // Example boarding passes from https://adventofcode.com/2020/day/5
        let boarding_pass = "BFFFBBFRRR";
        assert_eq!(boarding_pass_id(&boarding_pass), Some(567));

        let boarding_pass = "FFFBBBFRRR";
        assert_eq!(boarding_pass_id(&boarding_pass), Some(119));

        let boarding_pass = "BBFFBBFRLL";
        assert_eq!(boarding_pass_id(&boarding_pass), Some(820));
    }

    #[test]
    fn test_invalid_boarding_pass_id() {
        let boarding_pass = "BFFFBBFRR";
        assert_eq!(boarding_pass_id(&boarding_pass), None);

        let boarding_pass = "BFFFBBFRRQ";
        assert_eq!(boarding_pass_id(&boarding_pass), None);
    }
}
