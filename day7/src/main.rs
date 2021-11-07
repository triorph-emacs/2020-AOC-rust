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
    println!("Hello, world!");
    println!("test");
}

#[cfg(test)]
mod test {
    use crate::bag_parser;
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
}
