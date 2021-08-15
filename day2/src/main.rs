use anyhow::Result;
use std::convert::TryFrom;

struct PasswordLine {
    a: usize,
    b: usize,
    c: char,
    password: String,
}

#[derive(Debug)]
enum PasswordLineErrors {
    ParseInt(std::num::ParseIntError),
    Format,
}

impl PasswordLineErrors {
    fn from_parse_int(err: std::num::ParseIntError) -> PasswordLineErrors {
        PasswordLineErrors::ParseInt(err)
    }
}

impl PasswordLine {
    fn valid_day_a(&self) -> bool {
        let letter_count = self.password.matches(self.c).count();
        letter_count <= self.b && letter_count >= self.a
    }

    fn valid_day_b(&self) -> bool {
        let a_pos = self.password.chars().nth(self.a - 1);
        let b_pos = self.password.chars().nth(self.b - 1);
        match (a_pos, b_pos) {
            (Some(a_pos), Some(b_pos)) => (a_pos == self.c) ^ (b_pos == self.c),
            _ => false,
        }
    }
}

impl TryFrom<String> for PasswordLine {
    type Error = PasswordLineErrors;
    fn try_from(line: String) -> core::result::Result<Self, PasswordLineErrors> {
        // Format is "a-b c: password"
        let mut first_splitter = line.split(": ");
        let left = first_splitter.next().unwrap();
        let password = first_splitter.next();
        let mut left_splitter = left.split(" ");
        let values = left_splitter.next().unwrap();
        let c = left_splitter.next();
        let mut values_splitter = values.split("-");
        match (values_splitter.next(), values_splitter.next(), c, password) {
            (Some(a), Some(b), Some(c), Some(password))
                if (a != "" && b != "" && c != "" && password != "") =>
            {
                Ok(PasswordLine {
                    a: a.parse::<usize>()
                        .map_err(PasswordLineErrors::from_parse_int)?,
                    b: b.parse::<usize>()
                        .map_err(PasswordLineErrors::from_parse_int)?,
                    c: c.chars().next().unwrap(),
                    password: String::from(password),
                })
            }
            _ => Err(PasswordLineErrors::Format),
        }
    }
}

fn main() -> Result<()> {
    let s: Vec<PasswordLine> = include_str!("../input_data.txt")
        .lines()
        .map(String::from)
        .map(PasswordLine::try_from)
        .map(Result::unwrap)
        .collect();
    let day_a = count_valid_passwords_day_a(s.iter());
    println!("Day a valid passwords: {}", day_a);
    let day_b = count_valid_passwords_day_b(s.iter());
    println!("Day b valid passwords: {}", day_b);

    Ok(())
}

fn count_valid_passwords_day_a(password_lines: std::slice::Iter<PasswordLine>) -> usize {
    password_lines
        .filter(|password_line| password_line.valid_day_a())
        .count()
}

fn count_valid_passwords_day_b(password_lines: std::slice::Iter<PasswordLine>) -> usize {
    password_lines
        .filter(|password_line| password_line.valid_day_b())
        .count()
}
