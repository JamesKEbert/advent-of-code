use camino::Utf8PathBuf;

use clap::Subcommand;
use std::{
    collections::HashMap,
    error::{self},
    fmt::{self, Debug, Display},
    vec,
};

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day6Commands {
    /// Calculates Total Distinct Cells of guard path for given map
    Calculate {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
        /// Whether to test for valid obstructions
        #[arg(long, default_value_t = false)]
        valid_obstructions: bool,
    },
}

pub fn day6_cli_command_processing(command: &Day6Commands) {
    match command {
        Day6Commands::Calculate {
            path,
            valid_obstructions,
        } => {
            info!("Command received to calculate total distinct cells for guard");
            println!(
                "Total Number of Distinct Cells for guard's path: {}",
                count_distinct_cells(path.clone(), 10000,)
            );
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Entity {
    Empty,
    Obstruction,
    Guard(Guard),
}

#[derive(Clone, PartialEq, Debug)]
struct Guard {
    direction: Direction,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entity::Empty => write!(f, "."),
            Entity::Obstruction => write!(f, "#"),
            Entity::Guard(guard) => match guard.direction {
                Direction::North => write!(f, "^"),
                Direction::East => write!(f, ">"),
                Direction::South => write!(f, "V"),
                Direction::West => write!(f, "<"),
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Map {
    grid: Vec<Vec<Entity>>,
}
impl Map {
    pub fn new(grid: Vec<Vec<Entity>>) -> Self {
        Map { grid }
    }

    pub fn get_grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn get_width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.grid.len()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
enum Day6Error {
    NoGuard,
    GoingOutOfBounds,
}

impl fmt::Display for Day6Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Day6Error::NoGuard => write!(f, "no guard found in map"),
            Day6Error::GoingOutOfBounds => write!(f, "guard going out of bounds"),
        }
    }
}

impl error::Error for Day6Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Day6Error::NoGuard => None,
            Day6Error::GoingOutOfBounds => None,
        }
    }
}

fn parse_file(file_path: Utf8PathBuf) -> Map {
    info!("Parsing File");
    let content = read_file(file_path);

    let mut map: Map = Map::new(vec![]);
    let rows = content.split("\n");
    for (y, row) in rows.enumerate() {
        let mut parsed_row: Vec<Entity> = vec![];
        for (x, cell) in row.to_owned().chars().enumerate() {
            match cell {
                '.' => parsed_row.push(Entity::Empty),
                '#' => parsed_row.push(Entity::Obstruction),
                '^' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::North,
                    x,
                    y,
                })),
                '>' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::East,
                    x,
                    y,
                })),
                'V' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::South,
                    x,
                    y,
                })),
                '<' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::West,
                    x,
                    y,
                })),
                _ => panic!("Unidentified cell content in file: '{}'", cell),
            }
        }
        map.grid.push(parsed_row);
    }

    info!("Parsed Map:\n{}", map);

    map
}

fn find_guard(map: &Map) -> Option<Guard> {
    for (y, row) in map.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Entity::Guard(guard) = cell {
                trace!("Found guard {:?}", guard);
                return Some(guard.clone());
            }
        }
    }
    None
}

fn check_moving_out_of_bounds(map_width: usize, map_height: usize, guard: &Guard) -> bool {
    if guard.direction == Direction::North && guard.y == 0 {
        return true;
    }
    if guard.direction == Direction::East && guard.x == map_width - 1 {
        return true;
    }
    if guard.direction == Direction::South && guard.y == map_height - 1 {
        return true;
    }
    if guard.direction == Direction::West && guard.x == 0 {
        return true;
    }

    false
}

fn move_guard(mut map: Map, mut guard: &mut Guard) -> (Map, Guard) {
    // Remove existing guard spot from map
    map.grid[guard.y][guard.x] = Entity::Empty;

    if check_moving_out_of_bounds(map.get_width(), map.get_height(), &guard) {
        trace!("Moving Guard out of bounds");
        // There may be a better way to represent this, but this should be sufficient for this use case I think
        guard.x = usize::MAX;
        guard.y = usize::MAX;
        return (map, guard.to_owned());
    }

    // check next cell to determine if turn required, otherwise move forward
    match guard.direction {
        Direction::North => {
            if map.grid[guard.y - 1][guard.x] == Entity::Obstruction {
                guard.direction = Direction::East;
            } else {
                guard.y -= 1;
            }
        }
        Direction::East => {
            if map.grid[guard.y][guard.x + 1] == Entity::Obstruction {
                guard.direction = Direction::South;
            } else {
                guard.x += 1;
            }
        }
        Direction::South => {
            if map.grid[guard.y + 1][guard.x] == Entity::Obstruction {
                guard.direction = Direction::West;
            } else {
                guard.y += 1;
            }
        }
        Direction::West => {
            if map.grid[guard.y][guard.x - 1] == Entity::Obstruction {
                guard.direction = Direction::North;
            } else {
                guard.x -= 1;
            }
        }
    }
    map.grid[guard.y][guard.x] = Entity::Guard(guard.to_owned());

    (map, guard.to_owned())
}

fn simulate_patrol(mut map: Map, guard: &mut Guard, limit: i32) -> Vec<Guard> {
    let mut guard_positions = vec![];

    let mut iteration = 0;
    while iteration < limit {
        trace!("Iteration {}", iteration);
        if guard.x == usize::MAX || guard.y == usize::MAX {
            break;
        }

        guard_positions.push(guard.clone());
        (map, *guard) = move_guard(map, guard);
    }

    guard_positions
}

fn count_distinct_cells(file_path: Utf8PathBuf, simulation_limit: i32) -> usize {
    let map = parse_file(file_path);
    let mut guard = find_guard(&map).expect("guard to exist in map");
    let guard_positions = simulate_patrol(map, &mut guard, simulation_limit);

    let mut unique_positions: HashMap<Position, Guard> = HashMap::new();
    for guard in guard_positions {
        unique_positions.insert(
            Position {
                x: guard.x,
                y: guard.y,
            },
            guard,
        );
    }

    unique_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    fn create_empty_map() -> Map {
        let map: Map = Map::new(vec![
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
            vec![Entity::Empty; 10],
        ]);
        map
    }

    #[test]
    fn test_file_input() {
        test_init();

        let mut map = create_empty_map();
        map.grid[0][4] = Entity::Obstruction;
        map.grid[1][9] = Entity::Obstruction;
        map.grid[3][2] = Entity::Obstruction;
        map.grid[4][7] = Entity::Obstruction;
        map.grid[6][1] = Entity::Obstruction;
        map.grid[6][4] = Entity::Guard(Guard {
            direction: Direction::North,
            x: 4,
            y: 6,
        });
        map.grid[7][8] = Entity::Obstruction;
        map.grid[8][0] = Entity::Obstruction;
        map.grid[9][6] = Entity::Obstruction;
        assert_eq!(
            map,
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"))
        )
    }

    #[test]
    fn test_move_guard() {
        test_init();
        let mut original_guard = Guard {
            direction: Direction::North,
            x: 4,
            y: 5,
        };
        let mut map: Map = create_empty_map();
        map.grid[5][4] = Entity::Guard(original_guard.clone());
        debug!("Original Map: \n{}", map);

        let final_guard = Guard {
            direction: Direction::North,
            x: 4,
            y: 4,
        };
        let mut map_final = create_empty_map();
        map_final.grid[4][4] = Entity::Guard(final_guard.clone());
        debug!("Expected Map: \n{}", map_final);

        let (returned_map, _returned_guard) = move_guard(map, &mut original_guard);
        debug!("Returned Map: \n{}", returned_map);
        assert_eq!(
            map_final, returned_map,
            "Expect the guard to be one space to the north"
        );
    }

    #[test]
    fn test_simulate_patrol() {
        test_init();
        let mut original_guard = Guard {
            direction: Direction::North,
            x: 4,
            y: 2,
        };
        let mut map: Map = create_empty_map();
        map.grid[2][4] = Entity::Guard(original_guard.clone());
        debug!("Original Map: \n{}", map);

        assert_eq!(
            vec![
                Guard {
                    x: 4,
                    y: 2,
                    direction: Direction::North
                },
                Guard {
                    x: 4,
                    y: 1,
                    direction: Direction::North
                },
                Guard {
                    x: 4,
                    y: 0,
                    direction: Direction::North
                },
            ],
            simulate_patrol(map, &mut original_guard, 100),
            "Expected a vector of guard positions"
        );
    }

    #[test]
    fn test_simulate_patrol_sample_data() {
        test_init();

        assert_eq!(
            41,
            count_distinct_cells(
                Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"),
                10000
            )
        )
    }

    // #[test]
    // fn test_simulate_patrol_add_obstructions_sample_data() {
    //     test_init();

    //     assert_eq!(
    //         6,
    //         simulate_patrol(
    //             Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"),
    //             true
    //         )
    //         .expect("Simulation to work")
    //     )
    // }
}
