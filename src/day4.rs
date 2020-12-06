use std::collections::HashMap;

extern crate aoc_macros;
use aoc_macros::aoc_main;

mod aoc;

type Passport = HashMap<String, String>;

aoc_main!(parse_passports);

fn parse_passports(s: &str) -> Vec<Passport> {
    s.trim().split("\n\n").map(parse_passport).collect()
}

fn parse_passport(s: &str) -> Passport {
    let kvs: Vec<&str> = s.split(|c: char| c.is_whitespace()).collect();
    kvs.iter()
        .filter_map(|kv| {
            let kv: Vec<&str> = kv.split(':').collect();
            if kv.len() != 2 {
                None
            } else {
                Some((kv[0].trim().to_string(), kv[1].trim().to_string()))
            }
        })
        .collect()
}

fn part1(passports: &Vec<Passport>) {
    let nvalid = passports.iter().filter(|p| valid_fields(p)).count();
    println!("passports with valid fields: {}", nvalid);
}

fn part2(passports: &Vec<Passport>) {
    let nvalid = passports.iter().filter(|p| valid_passport(p)).count();
    println!("completely valid passports: {}", nvalid);
}

fn valid_fields(passport: &Passport) -> bool {
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    REQUIRED_FIELDS.iter().all(|&f| passport.contains_key(f))
}

fn valid_passport(passport: &Passport) -> bool {
    valid_fields(passport)
        && valid_byr(&passport["byr"])
        && valid_iyr(&passport["iyr"])
        && valid_eyr(&passport["eyr"])
        && valid_hgt(&passport["hgt"])
        && valid_hcl(&passport["hcl"])
        && valid_ecl(&passport["ecl"])
        && valid_pid(&passport["pid"])
}

fn valid_byr(val: &str) -> bool {
    is_between(val, 1920, 2002)
}

fn valid_iyr(val: &str) -> bool {
    is_between(val, 2010, 2020)
}

fn valid_eyr(val: &str) -> bool {
    is_between(val, 2020, 2030)
}

fn valid_hgt(val: &str) -> bool {
    (val.ends_with("cm") && is_between(&val[0..val.len() - 2], 150, 193))
        || (val.ends_with("in") && is_between(&val[0..val.len() - 2], 59, 76))
}

fn valid_hcl(val: &str) -> bool {
    val.len() == 7 && val.starts_with('#') && val[1..].chars().all(|c| c.is_digit(16))
}

fn valid_ecl(val: &str) -> bool {
    const ALLOWED_ECLS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    ALLOWED_ECLS.contains(&val)
}

fn valid_pid(val: &str) -> bool {
    val.len() == 9 && val.chars().all(|c| c.is_digit(10))
}

fn is_between(val: &str, lower: u32, upper: u32) -> bool {
    let val = val.parse::<u32>();
    val.is_ok() && {
        let val = val.unwrap();
        val >= lower && val <= upper
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_passport() {
        let passport_str = r"
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929";
        let passport = parse_passport(&passport_str);
        assert_eq!(
            passport,
            vec![
                ("iyr".to_string(), "2013".to_string()),
                ("ecl".to_string(), "amb".to_string()),
                ("cid".to_string(), "350".to_string()),
                ("eyr".to_string(), "2023".to_string()),
                ("pid".to_string(), "028048884".to_string()),
                ("hcl".to_string(), "#cfa07d".to_string()),
                ("byr".to_string(), "1929".to_string()),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_parse_passports() {
        let passports_str = r"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
            ";
        let passports = parse_passports(&passports_str);
        assert_eq!(passports.len(), 4);
    }

    #[test]
    fn test_valid_passport() {
        let passport = parse_passport(
            r"hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm",
        );
        assert_eq!(valid_fields(&passport), true);

        let passport = parse_passport(
            r"hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in",
        );
        assert_eq!(valid_fields(&passport), false);
    }

    #[test]
    fn test_valid_hcl() {
        let hcl = "#12af0b";
        assert!(valid_hcl(hcl));

        let hcl = "123abcd";
        assert!(!valid_hcl(hcl));

        let hcl = "#123ad";
        assert!(!valid_hcl(hcl));

        let hcl = "#123adx";
        assert!(!valid_hcl(hcl));
    }

    #[test]
    fn test_valid_pid() {
        let pid = "029320393";
        assert!(valid_pid(pid));

        let pid = "0293203932";
        assert!(!valid_pid(pid));

        let pid = "03203932";
        assert!(!valid_pid(pid));

        let pid = "03203932p";
        assert!(!valid_pid(pid));
    }

    #[test]
    fn test_invalid_passports() {
        let passport = parse_passport(
            r"eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        );
        assert!(!valid_passport(&passport));

        let passport = parse_passport(
            r"iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946",
        );
        assert!(!valid_passport(&passport));

        let passport = parse_passport(
            r"hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        );
        assert!(!valid_passport(&passport));

        let passport = parse_passport(
            r"hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007",
        );
        assert!(!valid_passport(&passport));
    }

    #[test]
    fn test_valid_passports() {
        let passport = parse_passport(
            r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f",
        );
        assert!(valid_passport(&passport));

        let passport = parse_passport(
            r"eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        );
        assert!(valid_passport(&passport));

        let passport = parse_passport(
            r"hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022",
        );
        assert!(valid_passport(&passport));

        let passport = parse_passport(
            r"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert!(valid_passport(&passport));
    }
}
