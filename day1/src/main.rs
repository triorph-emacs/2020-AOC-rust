use anyhow::Result;
use itertools::Itertools;
const TOTAL: i32 = 2020;

fn main() -> Result<()> {
    let s = include_str!("../input_data.txt");
    let lines = s.split('\n');
    let numbers: Vec<i32> = lines.map(str::parse::<i32>).map(Result::unwrap).collect();
    let (a, b) = get_pairs_from_numbers(&numbers).unwrap();
    println!(
        r#"part a: Our numbers are {:?} which multiply to {}"#,
        (a, b),
        a * b
    );
    let (a, b, c) = get_triples_from_numbers(&numbers).unwrap();
    println!(
        "part b: Our numbers are {:?} which multiply to {}",
        (a, b, c),
        a * b * c
    );
    Ok(())
}

fn get_pairs_from_numbers(numbers: &Vec<i32>) -> Option<(&i32, &i32)> {
    numbers
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| *a + *b == TOTAL)
}

fn get_triples_from_numbers(numbers: &Vec<i32>) -> Option<(&i32, &i32, &i32)> {
    numbers
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| *a + *b + *c == TOTAL)
}
