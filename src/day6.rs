extern crate bit_set;
use bit_set::BitSet;

extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

type Group<'a> = Vec<&'a str>;

aoc_main!(parse_groups);

fn parse_groups(input: &str) -> Vec<Group> {
    input
        .trim()
        .split("\n\n")
        .map(|g| g.trim().lines().collect())
        .collect()
}

fn part1(groups: &Vec<Group>) {
    let mut total = 0;
    for group in groups {
        total += count_group_responses_any(group);
    }
    println!("Total: {}", total);
}

fn part2(groups: &Vec<Group>) {
    let mut total = 0;
    for group in groups {
        total += count_group_responses_all(group);
    }
    println!("Total: {}", total);
}

fn count_group_responses_any(group: &Group) -> usize {
    let mut set: BitSet<u32> = BitSet::new();
    for person in group {
        person.chars().filter_map(char_to_bit_index).for_each(|c| {
            set.insert(c as usize);
        })
    }
    set.len()
}

fn count_group_responses_all(group: &Group) -> usize {
    let mut answers: BitSet<u32> = BitSet::from_bytes(&[0xFFu8; 8]);
    for person in group {
        let mut person_answers: BitSet<u32> = BitSet::new();
        person.chars().filter_map(char_to_bit_index).for_each(|c| {
            person_answers.insert(c as usize);
        });
        answers.intersect_with(&person_answers);
    }
    answers.len()
}

fn char_to_bit_index(c: char) -> Option<u8> {
    if c.is_ascii_alphabetic() {
        Some(((c.to_ascii_lowercase() as u32) - ('a' as u32)) as u8)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_group_responses_any() {
        // Test case from https://adventofcode.com/2020/day/6 part 1.
        let input = r"
            abcx
            abcy
            abcz
        ";
        let groups = parse_groups(input);
        assert_eq!(groups.len(), 1);

        assert_eq!(count_group_responses_any(&groups[0]), 6);
    }

    #[test]
    fn test_count_group_responses_all() {
        // Test case from https://adventofcode.com/2020/day/6 part 2.
        let input = r"
            abcx
            abcy
            abcz
        ";
        let groups = parse_groups(input);
        assert_eq!(groups.len(), 1);

        assert_eq!(count_group_responses_all(&groups[0]), 3);
    }

    #[test]
    fn test_char_to_bit_index() {
        assert_eq!(char_to_bit_index('a'), Some(0));
        assert_eq!(char_to_bit_index('z'), Some(25));
        assert_eq!(char_to_bit_index('9'), None);
    }
}
