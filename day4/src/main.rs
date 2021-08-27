#[derive(Debug, PartialEq)]
enum PassportField {
    IssueYear(String),
    EyeColour(String),
    PassportId(String),
    ExpiryYear(String),
    HairColour(String),
    BirthYear(String),
    Height(String),
}

#[derive(Debug)]
struct Passport {
    keys: Vec<PassportField>,
}

impl PassportField {
    fn new(key: &str, val: &str) -> Result<PassportField, String> {
        let val = String::from(val);
        match key {
            "byr" => Ok(PassportField::BirthYear(val)),
            "hgt" => Ok(PassportField::Height(val)),
            "hcl" => Ok(PassportField::HairColour(val)),
            "ecl" => Ok(PassportField::EyeColour(val)),
            "eyr" => Ok(PassportField::ExpiryYear(val)),
            "iyr" => Ok(PassportField::IssueYear(val)),
            "pid" => Ok(PassportField::PassportId(val)),
            _ => Err(String::from("Not a valid key")),
        }
    }
    fn _validate_pid(val: &str) -> bool {
        val.parse::<u32>().is_ok() && val.len() == 9
    }

    fn _validate_iyr(val: &str) -> bool {
        matches!(
                val.parse::<u32>(),
                Ok(val_u32) if (2010..=2020).contains(&val_u32)
        )
    }

    fn _validate_eyr(val: &str) -> bool {
        matches!(
                val.parse::<u32>(),
                Ok(val_u32) if (2020..=2030).contains(&val_u32)
        )
    }

    fn _validate_ecl(val: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
    }

    fn _is_lowercase_hex(c: &char) -> bool {
        ('0'..='9').contains(c) || ('a'..='f').contains(c)
    }

    fn _validate_hcl(val: &str) -> bool {
        if val.len() != 7 {
            return false;
        } else {
            for (i, c) in val.chars().enumerate() {
                if i == 0 && c != '#' {
                    return false;
                }
                if i > 0 && !PassportField::_is_lowercase_hex(&c) {
                    return false;
                }
            }
        }
        true
    }

    fn _validate_byr(val: &str) -> bool {
        matches!(
                val.parse::<u32>(),
                Ok(val_u32) if (1920..=2002).contains(&val_u32)
        )
    }

    fn _validate_hgt(val: &str) -> bool {
        if val.len() < 4 {
            return false;
        }
        let splitter = val.len() - 2;
        let unit = &val[splitter..];
        if unit == "cm" {
            return matches!(val[..splitter].parse::<u32>(),
        Ok(val_u32) if (150..=193).contains(&val_u32));
        } else if unit == "in" {
            return matches!(val[..splitter].parse::<u32>(),
        Ok(val_u32) if (59..=76).contains(&val_u32));
        }
        false
    }
    fn is_valid_day_b(&self) -> bool {
        match self {
            PassportField::BirthYear(val) => PassportField::_validate_byr(val),
            PassportField::ExpiryYear(val) => PassportField::_validate_eyr(val),
            PassportField::EyeColour(val) => PassportField::_validate_ecl(val),
            PassportField::IssueYear(val) => PassportField::_validate_iyr(val),
            PassportField::Height(val) => PassportField::_validate_hgt(val),
            PassportField::HairColour(val) => PassportField::_validate_hcl(val),
            PassportField::PassportId(val) => PassportField::_validate_pid(val),
        }
    }
}

impl Passport {
    fn parse(input: &str) -> Passport {
        let mut keys = Vec::<PassportField>::new();
        for line in input.lines() {
            for key_val in line.split(' ') {
                let mut key_val_split = key_val.split(':');
                let key = key_val_split.next().unwrap();
                let val = key_val_split
                    .next()
                    .expect("File must contain key:val format");

                if let Ok(valid_field) = PassportField::new(key, val) {
                    keys.push(valid_field);
                }
            }
        }
        Passport { keys }
    }

    fn is_valid_day_a(&self) -> bool {
        self.keys.len() == 7
    }

    fn is_valid_day_b(&self) -> bool {
        self.is_valid_day_a() && self.keys.iter().all(|key| key.is_valid_day_b())
    }
}

fn count_valid_day_a(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_day_a())
        .count()
}

fn count_valid_day_b(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_day_b())
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
    println!("Day a result: {}", valid_day_a);
    let valid_day_b = count_valid_day_b(&passports);
    println!("Day b result: {}", valid_day_b);
}

#[cfg(test)]
mod test {
    use crate::count_valid_day_a;
    use crate::count_valid_day_b;
    use crate::parse_data;
    use crate::PassportField;

    #[test]
    fn test_parse() {
        let data = include_str!("../test_data.txt");
        let ret = parse_data(data);
        assert_eq!(ret.len(), 4);
        for (i, (key, val)) in [
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("hgt", "183cm"),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(ret[0].keys[i], PassportField::new(key, val).unwrap());
        }
    }

    #[test]
    fn test_validity_day_a() {
        let data = include_str!("../test_data.txt");
        let ret = parse_data(data);
        assert_eq!(count_valid_day_a(&ret), 2);
    }

    #[test]
    fn test_validity_day_b() {
        let data = include_str!("../valid_dayb_passports.txt");
        let ret = parse_data(data);
        assert_eq!(count_valid_day_b(&ret), 4);
    }

    #[test]
    fn test_invalidity_day_b() {
        let data = include_str!("../invalid_dayb_passports.txt");
        let ret = parse_data(data);
        assert_eq!(count_valid_day_b(&ret), 0);
    }

    #[test]
    fn test_validity_valid_rules() {
        for (key, val) in [
            ("byr", "2002"),
            ("hgt", "60in"),
            ("hgt", "190cm"),
            ("hcl", "#123abc"),
            ("ecl", "brn"),
            ("pid", "000000001"),
        ] {
            let ret = PassportField::new(key, val).unwrap();
            assert!(
                ret.is_valid_day_b(),
                "Expected a pass for {} and {}",
                key,
                val
            );
        }
    }

    #[test]
    fn test_validity_invalid_rules() {
        for (key, val) in [
            ("byr", "2003"),
            ("hgt", "190in"),
            ("hgt", "190"),
            ("hcl", "#123abz"),
            ("hcl", "123abc"),
            ("ecl", "wat"),
            ("pid", "0123456789"),
        ] {
            let ret = PassportField::new(key, val).unwrap();
            assert!(
                !ret.is_valid_day_b(),
                "Expected a fail for {} and {}",
                key,
                val
            );
        }
    }
}
