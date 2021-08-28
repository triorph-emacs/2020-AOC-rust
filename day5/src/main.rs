// Day 5. Plane tickets are stored by a binary style system.
// 'B' means 1, 'F' means 0 for row.
// 'R' means 1, 'L' means 0 row column.
// Seat number is col * 8 + row.
//
// We have to find the biggest seat_number, and the
// seat_number not present in the full list

struct PlaneTicket {
    column: u32,
    row: u32,
}

impl PlaneTicket {
    fn make_binary_number(ticket_str: &str, zero_char: char, one_char: char) -> u32 {
        let mut ret: u32 = 0;
        for (i, c) in ticket_str.chars().enumerate() {
            ret <<= 1;
            match c {
                val if val == one_char => ret += 1,
                val if val == zero_char => (),
                invalid_char => {
                    panic!(
                        "Invalid character in plane ticket at position {}: {:?}",
                        i, invalid_char
                    );
                }
            }
        }
        ret
    }
    fn parse(val: &str) -> PlaneTicket {
        if val.len() != 10 {
            panic!("Invalid string for plane ticket {}", val)
        }
        let column = PlaneTicket::make_binary_number(&val[7..10], 'L', 'R');
        let row = PlaneTicket::make_binary_number(&val[0..7], 'F', 'B');
        PlaneTicket { column, row }
    }

    fn calc_seat(&self) -> u32 {
        self.column + self.row * 8
    }
}

fn calculate_day_a_answer(tickets: &[u32]) -> u32 {
    // find the max ticket
    return *tickets.iter().max().expect("There is at least one max");
}

fn calculate_day_b_answer(tickets: &[u32]) -> u32 {
    // Find the missing ticket in the middle
    let min = tickets.iter().min().expect("There is at least one min");
    let max = tickets.iter().max().expect("There is at least one max");
    for i in *min..*max {
        if !tickets.contains(&i) {
            return i;
        }
    }
    0
}

fn main() {
    let input = include_str!("../input_data.txt");
    let tickets = input
        .lines()
        .map(|line| PlaneTicket::parse(line).calc_seat())
        .collect::<Vec<u32>>();
    let tickets_ref = &tickets[..];

    println!("Day a answer: {}", calculate_day_a_answer(tickets_ref));
    println!("Day b answer: {}", calculate_day_b_answer(tickets_ref));
}

#[cfg(test)]
mod test {
    use crate::PlaneTicket;
    #[test]
    fn test_parse() {
        for (ticket_str, column, row) in [
            ("FBFBBFFRLR", 5, 44),
            ("BFFFBBFRRR", 7, 70),
            ("FFFBBBFRRR", 7, 14),
            ("BBFFBBFRLL", 4, 102),
        ] {
            let plane_ticket = PlaneTicket::parse(ticket_str);
            assert_eq!(plane_ticket.column, column);
            assert_eq!(plane_ticket.row, row);
        }
    }

    #[test]
    fn test_seat_number() {
        for (ticket_str, seat_number) in [
            ("FBFBBFFRLR", 357),
            ("BFFFBBFRRR", 567),
            ("FFFBBBFRRR", 119),
            ("BBFFBBFRLL", 820),
        ] {
            let plane_ticket = PlaneTicket::parse(ticket_str);
            assert_eq!(plane_ticket.calc_seat(), seat_number)
        }
    }
}
