use std::collections::HashMap;
const VALID_NAMES: [&str; 7] = ["iyr", "ecl", "pid", "eyr", "hcl", "byr", "hgt"];

#[derive(Debug)]
struct Passport {
    keys: HashMap<String, String>,
}

impl Passport {
    fn parse(input: &str) -> Passport {
        let mut keys = HashMap::<String, String>::new();
        for line in input.lines() {
            for key_val in line.split(' ') {
                let mut key_val_split = key_val.split(':');
                let key = key_val_split.next().unwrap();
                let val = key_val_split
                    .next()
                    .expect("File must contain key:val format");
                keys.insert(String::from(key), String::from(val));
            }
        }
        Passport { keys }
    }

    fn valid_day_a(&self) -> bool {
        for key in VALID_NAMES {
            if !self.keys.contains_key(key) {
                return false;
            }
        }
        true
    }
}

fn count_valid_day_a(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.valid_day_a())
        .count()
}

fn parse_data(input_data: &str) -> Vec<Passport> {
    let mut ret = Vec::<Passport>::new();
    for passport_block in input_data.split("\n\n") {
        ret.push(Passport::parse(passport_block));
    }
    ret
}

fn main() {
    let data = include_str!("../input_data.txt");
    let passports = parse_data(data);
    let valid_day_a = count_valid_day_a(&passports);
    println!("Day a result: {}", valid_day_a)
}

#[cfg(test)]
mod test {
    use crate::count_valid_day_a;
    use crate::parse_data;

    #[test]
    fn test_parse() {
        let data = include_str!("../test_data.txt");
        let ret = parse_data(data);
        assert_eq!(ret.len(), 4);
        for (key, val) in [
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("cid", "147"),
            ("hgt", "183cm"),
        ] {
            assert_eq!(ret[0].keys.get(key), Some(&String::from(val)));
        }
    }

    #[test]
    fn test_validity_day_a() {
        let data = include_str!("../test_data.txt");
        let ret = parse_data(data);
        assert_eq!(count_valid_day_a(&ret), 2);
    }
}
