extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

#[derive(Copy, Clone, Debug, PartialEq)]
struct PasswordRule {
    c: char,
    min: u32,
    max: u32,
}

aoc_main!();

fn part1(input: &str) {
    let count = count_valid_old(input);
    println!("valid: {}", count);
}

fn part2(input: &str) {
    let count = count_valid_new(input);
    println!("valid: {}", count);
}

fn count_valid(s: &str, f: fn(&PasswordRule, &str) -> bool) -> u32 {
    parse_lines(s).iter().filter(|(rule, s)| f(rule, s)).count() as u32
}

fn count_valid_old(s: &str) -> u32 {
    count_valid(s, is_valid_old)
}

fn count_valid_new(s: &str) -> u32 {
    count_valid(s, is_valid_new)
}

fn parse_lines(s: &str) -> Vec<(PasswordRule, String)> {
    s.trim().split('\n').map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (PasswordRule, String) {
    let components = line.trim().split(": ").collect::<Vec<&str>>();
    let rule = parse_rule(components[0]);
    (rule, components[1].to_string())
}

fn parse_rule(rule: &str) -> PasswordRule {
    let components = rule.split('-').collect::<Vec<&str>>();
    let min: u32 = components[0].parse::<u32>().expect("Bad min value");
    let components = components[1].split(' ').collect::<Vec<&str>>();
    let max: u32 = components[0].parse::<u32>().expect("Bad max value");
    PasswordRule {
        c: components[1].chars().next().unwrap(),
        min,
        max,
    }
}

fn is_valid_old(rule: &PasswordRule, s: &str) -> bool {
    let count = s.chars().filter(|&c| c == rule.c).count() as u32;
    count >= rule.min && count <= rule.max
}

fn is_valid_new(rule: &PasswordRule, s: &str) -> bool {
    let len = s.len() as u32;
    let chars: Vec<char> = s.chars().collect();
    (len >= rule.min && chars[(rule.min - 1) as usize] == rule.c)
        ^ (len >= rule.max && chars[(rule.max - 1) as usize] == rule.c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let input = "1-3 a";
        let rule = parse_rule(input);
        assert_eq!(
            rule,
            PasswordRule {
                c: 'a',
                min: 1,
                max: 3
            }
        );
    }

    #[test]
    fn test_parse_line() {
        let input = "1-3 b: cdefg";
        let (rule, s) = parse_line(input);
        assert_eq!(
            rule,
            PasswordRule {
                c: 'b',
                min: 1,
                max: 3
            }
        );
        assert_eq!(s, "cdefg".to_string());
    }

    #[test]
    fn test_count_valid_old() {
        let input = r"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";
        let count = count_valid_old(&input);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_count_valid_new() {
        let input = r"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";
        let count = count_valid_new(&input);
        assert_eq!(count, 1);
    }
}
