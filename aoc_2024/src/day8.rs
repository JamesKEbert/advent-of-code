use std::collections::{HashMap, HashSet};

use camino::Utf8PathBuf;
use clap::Subcommand;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day8Commands {
    /// Generates and totals unique antinodes
    GenerateAntinodes {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
        /// Whether to account for resonant harmonics
        #[arg(long, default_value_t = false)]
        harmonics: bool,
    },
}

pub fn day8_cli_command_processing(command: &Day8Commands) {
    match command {
        Day8Commands::GenerateAntinodes { path, harmonics } => {
            info!("Command received to generate and total antinodes");
            println!(
                "Total unique antinodes: {}",
                calculate_all_antinodes(path.clone(), harmonics.to_owned())
            );
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
    frequency: char,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
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

fn calculate_limitless_antinodes(
    antenna1: &Position,
    antenna2: &Position,
    harmonize: bool,
    map_width: i32,
    map_height: i32,
) -> Vec<Position> {
    let width = (antenna1.x - antenna2.x).abs();
    let height = (antenna1.y - antenna2.y).abs();

    debug!("Calculated Width: {} Calculated Height: {}", width, height);
    let mut antinodes = vec![];

    // top left
    let mut x = antenna1.x;
    let mut y = antenna1.y;
    debug!("(x,y) ({},{})", x, y);
    while x >= 0 && y >= 0 && antenna1.y <= antenna2.y && antenna1.x <= antenna2.x {
        debug!("Top Left Antinode");
        antinodes.push(Position {
            x,
            y,
            frequency: antenna1.frequency,
        });
        x -= width;
        y -= height;
    }

    debug!("(x,y) ({},{})", x, y);
    // top right
    x = antenna1.x;
    y = antenna1.y;

    debug!("(x,y) ({},{})", x, y);
    while x < map_width && y >= 0 && antenna1.y < antenna2.y && antenna1.x > antenna2.x {
        debug!("Top Right Antinode");
        antinodes.push(Position {
            x,
            y,
            frequency: antenna1.frequency,
        });
        x += width;
        y -= height;
    }

    // bottom left
    x = antenna1.x;
    y = antenna1.y;
    while x >= 0 && y < map_height && antenna1.y > antenna2.y && antenna1.x < antenna2.x {
        debug!("Bottom Left Antinode");
        antinodes.push(Position {
            x,
            y,
            frequency: antenna1.frequency,
        });
        x -= width;
        y += height;
    }

    // bottom right
    x = antenna1.x;
    y = antenna1.y;
    while x < map_width && y < map_height && antenna1.y > antenna2.y && antenna1.x > antenna2.x {
        debug!("Bottom Right Antinode");
        antinodes.push(Position {
            x,
            y,
            frequency: antenna1.frequency,
        });
        x += width;
        y += height;
    }
    debug!("Antinodes: {:?}", antinodes);
    antinodes
}

fn calculate_antinodes(antenna1: Position, antenna2: Position) -> (Position, Position) {
    let width = (antenna1.x - antenna2.x).abs();
    let height = (antenna1.y - antenna2.y).abs();

    debug!("Calculated Width: {} Calculated Height: {}", width, height);
    let antinode1: Position;
    let antinode2: Position;
    // antenna1 is top left or they are in a line
    if antenna1.y <= antenna2.y && antenna1.x <= antenna2.x {
        trace!("Antenna1 is top left");
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
        trace!("Antenna1 is top right");
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
        trace!("Antenna1 is bottom left");
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
        trace!("Antenna1 is bottom right");
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
    debug!(
        "Antinodes are at: {:?},{:?}",
        (antinode1.x, antinode1.y),
        (antinode2.x, antinode2.y)
    );
    (antinode1, antinode2)
}

fn calculate_all_antinodes(file_path: Utf8PathBuf, harmonize: bool) -> usize {
    let (map, width, height) = parse_file(file_path);
    let mut antinode_map: HashSet<Position> = HashSet::new();

    for hash_set in map {
        if hash_set.len() > 1 {
            for antenna1 in &hash_set {
                for antenna2 in &hash_set {
                    if antenna1 == antenna2 {
                        continue;
                    }
                    if harmonize {
                        let antinodes = calculate_limitless_antinodes(
                            antenna1,
                            antenna2,
                            harmonize,
                            width as i32,
                            height as i32,
                        );
                        for node in antinodes {
                            antinode_map.insert(node);
                        }
                    } else {
                        let (antinode1, antinode2) =
                            calculate_antinodes(antenna1.to_owned(), antenna2.to_owned());
                        antinode_map.insert(antinode1);
                        antinode_map.insert(antinode2);
                    }
                }
            }
        }
    }

    antinode_map.retain(|position| {
        position.x >= 0
            && position.x < width as i32
            && position.y >= 0
            && position.y < height as i32
    });

    let mut deduped_antinode_map: HashSet<Coordinates> = HashSet::new();
    for node in antinode_map {
        deduped_antinode_map.insert(Coordinates {
            x: node.x,
            y: node.y,
        });
    }

    display_map(&deduped_antinode_map, width, height);
    debug!("Antinode map {:?}", deduped_antinode_map);
    deduped_antinode_map.len()
}

fn display_map(map: &HashSet<Coordinates>, width: usize, height: usize) -> () {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for position in map {
        grid[position.y as usize][position.x as usize] = '#';
    }
    let mut grid_string = "".to_string();
    for row in grid {
        let row_string: String = row.iter().collect();
        grid_string = [grid_string, row_string, "\n".to_string()].concat();
    }
    info!("Map: [\n{}\n]", grid_string);
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
            calculate_all_antinodes(
                Utf8PathBuf::from("./src/puzzle_inputs/day8_sample2.txt"),
                false
            )
        )
    }

    #[test]
    fn test_calculate_all_antinodes_from_sample2_harmonize() {
        test_init();
        assert_eq!(
            34,
            calculate_all_antinodes(
                Utf8PathBuf::from("./src/puzzle_inputs/day8_sample2.txt"),
                true
            )
        )
    }
}
