use std::collections::{HashMap, HashSet};

use camino::Utf8PathBuf;

use crate::read_file;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    frequency: char,
}

fn parse_file(file_path: Utf8PathBuf) -> (Vec<HashSet<Position>>, usize, usize) {
    let mut map: Vec<HashSet<Position>> = vec![];
    let mut frequency_index: HashMap<char, usize> = HashMap::new();

    let content = read_file(file_path);
    let string_map: Vec<&str> = content.split("\n").collect();
    let height = string_map.len();
    let width = string_map[0].len();
    for (y, row) in string_map.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char != '.' {
                let antenna = Position {
                    x: x as i32,
                    y: y as i32,
                    frequency: char,
                };

                let index = frequency_index.get(&char);
                if index.is_some() {
                    map[index.expect("expect to be valid index").to_owned()].insert(antenna);
                } else {
                    let mut hash_set: HashSet<Position> = HashSet::new();
                    hash_set.insert(antenna);
                    map.push(hash_set);
                    frequency_index.insert(char, map.len() - 1);
                }
            }
        }
    }

    (map, width, height)
}

fn calculate_antinodes(antenna1: Position, antenna2: Position) -> (Position, Position) {
    let width = (antenna1.x - antenna2.x).abs();
    let height = (antenna1.y - antenna2.y).abs();

    debug!("Calculated Width: {} Calculated Height: {}", width, height);
    let antinode1: Position;
    let antinode2: Position;
    // antenna1 is top left or they are in a line
    if antenna1.y <= antenna2.y && antenna1.x <= antenna2.x {
        debug!("Antenna1 is top left");
        antinode1 = Position {
            x: antenna1.x - width,
            y: antenna1.y - height,
            frequency: antenna1.frequency,
        };
        antinode2 = Position {
            x: antenna2.x + width,
            y: antenna2.y + height,
            frequency: antenna1.frequency,
        };
    }
    // antenna1 is top right
    else if antenna1.y < antenna2.y && antenna1.x > antenna2.x {
        debug!("Antenna1 is top right");
        antinode1 = Position {
            x: antenna1.x + width,
            y: antenna1.y - height,
            frequency: antenna1.frequency,
        };
        antinode2 = Position {
            x: antenna2.x - width,
            y: antenna2.y + height,
            frequency: antenna1.frequency,
        };
    }
    // antenna1 is bottom left
    else if antenna1.y > antenna2.y && antenna1.x < antenna2.x {
        debug!("Antenna1 is bottom left");
        antinode1 = Position {
            x: antenna1.x - width,
            y: antenna1.y + height,
            frequency: antenna1.frequency,
        };
        antinode2 = Position {
            x: antenna2.x + width,
            y: antenna2.y - height,
            frequency: antenna1.frequency,
        };
    }
    // antenna1 is bottom right
    // if antenna1.y > antenna2.y && antenna1.x > antenna2.x {
    else {
        debug!("Antenna1 is bottom right");
        antinode1 = Position {
            x: antenna1.x + width,
            y: antenna1.y + height,
            frequency: antenna1.frequency,
        };
        antinode2 = Position {
            x: antenna2.x - width,
            y: antenna2.y - height,
            frequency: antenna1.frequency,
        };
    }
    (antinode1, antinode2)
}

fn calculate_all_antinodes(file_path: Utf8PathBuf) -> usize {
    let (map, width, height) = parse_file(file_path);

    0
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use super::*;
    use crate::test_init;

    #[test]
    fn test_file_input() {
        test_init();
        let mut hash_set: HashSet<Position> = HashSet::new();
        hash_set.insert(Position {
            x: 4,
            y: 3,
            frequency: 'a',
        });
        hash_set.insert(Position {
            x: 8,
            y: 4,
            frequency: 'a',
        });
        hash_set.insert(Position {
            x: 5,
            y: 5,
            frequency: 'a',
        });
        let mut hash_set2: HashSet<Position> = HashSet::new();
        hash_set2.insert(Position {
            x: 6,
            y: 7,
            frequency: 'A',
        });
        let map = vec![hash_set, hash_set2];
        assert_eq!(
            (map, 10, 10),
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day8_sample1.txt"))
        )
    }

    #[test]
    fn test_antinode_creator() {
        test_init();

        let antenna1 = Position {
            x: 4,
            y: 3,
            frequency: 'a',
        };
        let antenna2 = Position {
            x: 5,
            y: 5,
            frequency: 'a',
        };
        assert_eq!(
            (
                Position {
                    x: 3,
                    y: 1,
                    frequency: 'a'
                },
                Position {
                    x: 6,
                    y: 7,
                    frequency: 'a'
                }
            ),
            calculate_antinodes(antenna1, antenna2)
        )
    }

    #[test]
    fn test_calculate_all_antinodes_from_sample2() {
        test_init();
        assert_eq!(
            14,
            calculate_all_antinodes(Utf8PathBuf::from("./src/puzzle_inputs/day8_sample2.txt"))
        )
    }
}
