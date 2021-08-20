#[derive(std::hash::Hash, PartialEq, Eq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct SkiMap {
    width: usize,
    height: usize,
    positions: std::collections::HashMap<Point, bool>,
}

impl std::fmt::Display for SkiMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                ret += match self.get(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    Some(true) => "#",
                    _ => ".",
                };
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}

impl From<&[&[u8]]> for SkiMap {
    fn from(lines: &[&[u8]]) -> SkiMap {
        let mut positions = std::collections::HashMap::<Point, bool>::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                positions.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    *c == b'#',
                );
            }
        }
        let width = lines[0].len();
        let height = lines.len();
        println!("Creating skimap with width {}, height {}", width, height);
        SkiMap {
            width,
            height,
            positions,
        }
    }
}

impl From<&str> for SkiMap {
    fn from(input: &str) -> SkiMap {
        let lines = &input.lines().map(str::as_bytes).collect::<Vec<&[u8]>>()[..];
        SkiMap::from(lines)
    }
}

impl SkiMap {
    fn get(&self, position: &Point) -> Option<bool> {
        let width = self.width as isize;
        let mapped_point = Point {
            x: (((position.x % width) + width) % width),
            ..*position
        };
        self.positions.get(&mapped_point).copied()
    }
    fn count_vector(&self, movement: &Point) -> usize {
        let mut position = Point { x: 0, y: 0 };
        let mut count = 0;
        loop {
            match self.get(&position) {
                Some(true) => count += 1,
                Some(false) => (),
                None => break,
            };
            position = Point {
                x: position.x + movement.x,
                y: position.y + movement.y,
            };
        }
        count
    }
    fn day_b_calculate(&self) -> usize {
        vec![
            Point { x: 1, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 5, y: 1 },
            Point { x: 7, y: 1 },
            Point { x: 1, y: 2 },
        ]
        .iter()
        .map(|x| self.count_vector(x))
        .product()
    }
}

fn main() {
    let lines: &str = include_str!("../input_data.txt");
    let ski_map = SkiMap::from(lines);
    println!("Parsed ski_map: {}", ski_map);
    println!("Calculating day_a:");
    let day_a_tree_count = ski_map.count_vector(&Point { x: 3, y: 1 });
    println!(
        "Day a: hit {} trees on path down the mountain.",
        day_a_tree_count
    );
    let day_b_tree_multiplier = ski_map.day_b_calculate();
    println!("Day b: Our multiplier result is {}", day_b_tree_multiplier);
}

#[cfg(test)]
mod test {

    use crate::Point;
    use crate::SkiMap;
    #[test]
    fn test_get_function() {
        let lines: &[&[u8]] = &[&[b'.', b'#'][..], &[b'#', b'.'][..]][..];
        let map = SkiMap::from(lines);
        println!("creating skimap {:?}", map);
        assert_eq!(map.get(&Point { x: 0, y: 0 }), Some(false));
        assert_eq!(map.get(&Point { x: 0, y: -1 }), None);
        assert_eq!(map.get(&Point { x: 0, y: 3 }), None);
        assert_eq!(map.get(&Point { x: 1, y: 0 }), Some(true));
        assert_eq!(map.get(&Point { x: 0, y: 1 }), Some(true));
        assert_eq!(map.get(&Point { x: -2, y: 0 }), Some(false));
        assert_eq!(map.get(&Point { x: -1, y: 0 }), Some(true));
        assert_eq!(map.get(&Point { x: 4, y: 1 }), Some(true));
        assert_eq!(map.get(&Point { x: 8, y: 1 }), Some(true));
    }

    #[test]
    fn test_path() {
        let lines: &str = include_str!("../test_data.txt");
        let map = SkiMap::from(lines);
        assert_eq!(map.count_vector(&Point { x: 3, y: 1 }), 7);
    }

    #[test]
    fn test_day_b_calculation() {
        let lines: &str = include_str!("../test_data.txt");
        let map = SkiMap::from(lines);
        assert_eq!(map.day_b_calculate(), 336);
    }
}
