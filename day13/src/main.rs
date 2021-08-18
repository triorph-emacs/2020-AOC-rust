#[derive(Debug)]
struct Bus {
    arrival: u32,
    frequency: u32,
}

#[derive(Debug)]
struct BusSchedule {
    start_time: u32,
    buses: Vec<Bus>,
}
impl From<Vec<&str>> for BusSchedule {
    fn from(lines: Vec<&str>) -> BusSchedule {
        let mut lines = lines.iter();
        let start_time = lines
            .next()
            .unwrap()
            .parse::<u32>()
            .expect("First line should be an integer");
        let mut buses = Vec::<Bus>::new();

        for (i, value) in lines.next().unwrap().split(',').enumerate() {
            if value != "x" {
                buses.push(Bus {
                    arrival: i as u32,
                    frequency: value.parse().expect("Values must be integers"),
                })
            }
        }
        BusSchedule { start_time, buses }
    }
}

impl BusSchedule {
    fn calculate_time(&self) -> u64 {
        let mut increment: u64 = 1;
        let mut calculated_time: u64 = 0;
        for bus in self.buses.iter() {
            println!("Finding time for bus {:?}", bus);
            loop {
                println!(
                    "Calculated time {}, bus {:?}, {}",
                    calculated_time,
                    bus,
                    calculated_time % (bus.frequency as u64)
                );
                if (calculated_time % (bus.frequency as u64))
                    == ((bus.frequency - bus.arrival) % bus.frequency) as u64
                {
                    println!("Found time {}", calculated_time);
                    increment *= bus.frequency as u64;
                    break;
                }
                calculated_time += increment;
            }
        }
        calculated_time
    }
}

fn main() {
    let lines: Vec<&str> = include_str!("../input_data.txt").lines().collect();
    let bus_schedule = BusSchedule::from(lines);
    bus_schedule.calculate_time();
}

#[cfg(test)]
mod test {
    use crate::BusSchedule;
    #[test]
    fn test_parsing() {
        let lines: Vec<&str> = vec!["939", "7,13,x,x,59,x,31,19"];
        let bus_schedule: BusSchedule = BusSchedule::from(lines);
        assert_eq!(bus_schedule.start_time, 939);
        assert_eq!(bus_schedule.buses.len(), 5);
        let buses = &bus_schedule.buses[..];
        for (i, (frequency, arrival)) in [(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]
            .iter()
            .enumerate()
        {
            assert_eq!(buses[i].frequency, *frequency);
            assert_eq!(buses[i].arrival, *arrival);
        }
    }

    #[test]
    fn test_calculate_time() {
        for (schedule_string, expected_time) in [
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ]
        .iter()
        {
            assert_eq!(
                BusSchedule::from(vec!["0", schedule_string]).calculate_time(),
                *expected_time
            );
        }
    }
}
