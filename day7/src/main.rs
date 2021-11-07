extern crate peg;
#[derive(Debug)]
pub struct BagQuantity {
    colour: String,
    quantity: usize,
}
#[derive(Debug)]
pub struct Bag {
    colour: String,
    contains: Vec<BagQuantity>,
}
fn find_parents_for_colour<'a>(bags: &'a [Bag], bag_to_calculate: &str) -> Vec<&'a str> {
    let mut ret: Vec<&'a str> = vec![];
    for bag in bags.iter() {
        for child_bag in bag.contains.iter() {
            if child_bag.colour == bag_to_calculate {
                ret.push(&bag.colour[..]);
            }
        }
    }
    ret
}
fn calculate_day_a(bags: &[Bag], bag_to_calculate: &str) -> usize {
    let mut count = 0;
    let mut bags_found: Vec<&str> = vec![bag_to_calculate];

    while count < bags_found.len() {
        let bag = bags_found[count];
        let new_bags = find_parents_for_colour(bags, bag);
        for new_bag in new_bags.iter() {
            if !bags_found.contains(new_bag) {
                bags_found.push(new_bag);
            }
        }
        count += 1;
    }

    count - 1 // don't include the starting bag
}

// Parse the each input line into a Bag object.
// Bags have a colour, and can contain other bags of given colours.
//
// The line to parse follows this format:
// green bags contain 4 red bags.
// red bags contain 2 blue bags, 1 yellow bag, 5 purple yellow bags.
// blue bags contain no other bags.
// yellow bags contain 1 shiny blue bag.
// purple yellow bags contain no other bags.
// shiny blue bags contain no other bags.
peg::parser! { grammar bag_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) {n.parse().unwrap()}

    rule bag_quantity() -> BagQuantity
        = count:number() " " colour:colour() " bag" "s"? "."? {
            BagQuantity{ colour, quantity: count }
        }

    rule inner_bags() -> Vec<BagQuantity>
        = b:bag_quantity() ** ", " { b }

    rule no_other_bags() -> Vec<BagQuantity>
        = "no other bags." { vec![] }

    rule not_bag() -> ()
        = !(" bag")

    rule colour() -> String
        = c:$((not_bag()[^'0'..='9'])+) {c.to_string()}

    pub rule parse() -> Bag
        = colour:colour() (" bags contain ") contains:(no_other_bags() / inner_bags()) {
            Bag{ colour, contains }
        }
}}

fn main() {
    let lines: &str = include_str!("../input_data.txt");
    let bags: Vec<Bag> = lines
        .lines()
        .map(bag_parser::parse)
        .map(Result::unwrap)
        .collect();
    println!("Day a result: {}", calculate_day_a(&bags, "shiny gold"));
}

#[cfg(test)]
mod test {
    use crate::bag_parser;
    use crate::calculate_day_a;
    use crate::Bag;
    #[test]
    fn test_parse_bag() {
        let bag = bag_parser::parse("green bags contain 2 red bags, 4 blue bags.");
        if let Ok(bag) = bag {
            assert_eq!(bag.colour, "green");
            assert_eq!(bag.contains.len(), 2);
            assert_eq!(bag.contains[0].colour, "red");
            assert_eq!(bag.contains[0].quantity, 2);
            assert_eq!(bag.contains[1].colour, "blue");
            assert_eq!(bag.contains[1].quantity, 4);
        } else {
            // consider testing with `cargo test --features peg/trace`
            panic!("bag had an error, {:?}", bag);
        }
    }

    #[test]
    fn test_parse_bag_larger_colours() {
        let bag = bag_parser::parse("dark green bags contain 2 dull red bags, 1 blueish grey bag.");
        if let Ok(bag) = bag {
            assert_eq!(bag.colour, "dark green");
            assert_eq!(bag.contains.len(), 2);
            assert_eq!(bag.contains[0].colour, "dull red");
            assert_eq!(bag.contains[0].quantity, 2);
            assert_eq!(bag.contains[1].colour, "blueish grey");
            assert_eq!(bag.contains[1].quantity, 1);
        } else {
            panic!("bag had an error, {:?}", bag);
        }
    }

    #[test]
    fn test_parse_bag_no_bags_inside() {
        let bag = bag_parser::parse("faded blue bags contain no other bags.");
        if let Ok(bag) = bag {
            assert_eq!(bag.colour, "faded blue");
            assert_eq!(bag.contains.len(), 0);
        } else {
            panic!("bag had an error, {:?}", bag);
        }
    }

    #[test]
    fn test_day_a_problem() {
        let lines: &str = include_str!("../test_data_a.txt");
        let bags: Vec<Bag> = lines
            .lines()
            .map(bag_parser::parse)
            .map(Result::unwrap)
            .collect();
        assert_eq!(calculate_day_a(&bags, "shiny gold"), 4);
    }
}
