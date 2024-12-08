use camino::Utf8PathBuf;

use clap::Subcommand;
use std::{
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
                simulate_patrol(path.clone(), valid_obstructions.to_owned())
                    .expect("Simulations to work")
            );
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Entity {
    Empty,
    Obstruction,
    Guard(Guard),
    Path,
}

#[derive(Clone, PartialEq, Debug)]
struct Guard {
    direction: Direction,
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
            Entity::Path => write!(f, "X"),
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

struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{}]", self.x, self.y)
    }
}

fn parse_file(file_path: Utf8PathBuf) -> Map {
    info!("Parsing File");
    let content = read_file(file_path);

    let mut map: Map = Map::new(vec![]);
    let rows = content.split("\n");
    for row in rows {
        let mut parsed_row: Vec<Entity> = vec![];
        for cell in row.to_owned().chars() {
            match cell {
                '.' => parsed_row.push(Entity::Empty),
                '#' => parsed_row.push(Entity::Obstruction),
                '^' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::North,
                })),
                '>' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::East,
                })),
                'V' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::South,
                })),
                '<' => parsed_row.push(Entity::Guard(Guard {
                    direction: Direction::West,
                })),
                _ => panic!("Unidentified cell content in file: '{}'", cell),
            }
        }
        map.grid.push(parsed_row);
    }

    info!("Parsed Map:\n{}", map);

    map
}

fn find_guard(map: &Map) -> Option<(Position, Guard)> {
    for (y, row) in map.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Entity::Guard(guard) = cell {
                let position = Position { x, y };
                debug!("Found guard at {}", position);
                return Some((position, guard.clone()));
            }
        }
    }
    None
}
// This is a little inefficient given that we aren't keeping track of the guard's position
fn progress_guard(mut map: Map, trail: bool) -> Result<Map, Day6Error> {
    let (guard_position, guard) = find_guard(&map).ok_or(Day6Error::NoGuard)?;

    debug!("Guard Moving '{:?}'", guard.direction);
    // Remove from current position
    if trail {
        map.grid[guard_position.y][guard_position.x] = Entity::Path;
    } else {
        map.grid[guard_position.y][guard_position.x] = Entity::Empty;
    }
    // Out of bounds check
    // Determine new direction
    let mut new_direction = guard.direction.clone();
    let mut turned = false;
    match guard {
        Guard {
            direction: Direction::North,
        } => {
            // Out of bounds check, if so, just return map
            if guard_position.y == 0 {
                return Ok(map);
            }
            // Determine if we turn right
            if map.grid[guard_position.y - 1][guard_position.x] == Entity::Obstruction {
                new_direction = Direction::East;
                turned = true;
            }
        }
        Guard {
            direction: Direction::East,
        } => {
            // Out of bounds check, if so, just return map
            if guard_position.x == map.get_width() - 1 {
                return Ok(map);
            }
            // Determine if we turn right
            if map.grid[guard_position.y][guard_position.x + 1] == Entity::Obstruction {
                new_direction = Direction::South;
                turned = true;
            }
        }
        Guard {
            direction: Direction::South,
        } => {
            // Out of bounds check, if so, just return map
            if guard_position.y == map.get_height() - 1 {
                return Ok(map);
            }
            // Determine if we turn right
            if map.grid[guard_position.y + 1][guard_position.x] == Entity::Obstruction {
                new_direction = Direction::West;
                turned = true;
            }
        }
        Guard {
            direction: Direction::West,
        } => {
            // Out of bounds check, if so, just return map
            if guard_position.x == 0 {
                return Ok(map);
            }
            // Determine if we turn right
            if map.grid[guard_position.y][guard_position.x - 1] == Entity::Obstruction {
                new_direction = Direction::North;
                turned = true;
            }
        }
    }

    debug!("Guard's New Direction '{:?}'", new_direction);

    if turned {
        map.grid[guard_position.y][guard_position.x] = Entity::Guard(Guard {
            direction: new_direction.clone(),
        })
    } else {
        // Place Guard in new direction cell
        match new_direction {
            Direction::North => {
                map.grid[guard_position.y - 1][guard_position.x] = Entity::Guard(Guard {
                    direction: new_direction,
                })
            }
            Direction::East => {
                map.grid[guard_position.y][guard_position.x + 1] = Entity::Guard(Guard {
                    direction: new_direction,
                })
            }
            Direction::South => {
                map.grid[guard_position.y + 1][guard_position.x] = Entity::Guard(Guard {
                    direction: new_direction,
                })
            }
            Direction::West => {
                map.grid[guard_position.y][guard_position.x - 1] = Entity::Guard(Guard {
                    direction: new_direction,
                })
            }
        }
    }
    debug!("Updated Map: \n{}", map);

    Ok(map)
}

fn simulate_patrol(file_path: Utf8PathBuf, test_add_obstructions: bool) -> Result<i32, Day6Error> {
    info!("Simulating Patrol");
    let mut map = parse_file(file_path);
    let original_map = map.clone();
    info!("Map:\n{}", map);

    let mut valid_obstruction_count = 0;

    let mut iteration = 0;

    while find_guard(&map).is_some() {
        info!("Map:\n{}", map);
        let (guard_position, guard) = find_guard(&map).ok_or(Day6Error::NoGuard)?;
        if test_add_obstructions {
            if test_add_obstruction(original_map.clone(), guard_position, guard)? {
                valid_obstruction_count += 1
            }
        }
        map = progress_guard(map, !test_add_obstructions)?;
        iteration += 1;
        println!(
            "Iteration {}, valid obstruction count {}",
            iteration, valid_obstruction_count
        );
    }

    if test_add_obstructions {
        Ok(valid_obstruction_count)
    } else {
        println!("Total Iterations: {}", iteration);
        Ok(calculate_unique_cells(&map))
    }
}

fn test_add_obstruction(
    mut map: Map,
    guard_position: Position,
    guard: Guard,
) -> Result<bool, Day6Error> {
    info!("Test adding obstruction ahead");

    match guard.direction {
        Direction::North => {
            if guard_position.y != 0 {
                if map.grid[guard_position.y - 1][guard_position.x] == Entity::Empty {
                    map.grid[guard_position.y - 1][guard_position.x] = Entity::Obstruction;
                }
            }
        }
        Direction::East => {
            if guard_position.x != map.get_width() - 1 {
                if map.grid[guard_position.y][guard_position.x + 1] == Entity::Empty {
                    map.grid[guard_position.y][guard_position.x + 1] = Entity::Obstruction;
                }
            }
        }
        Direction::South => {
            if guard_position.y != map.get_height() - 1 {
                if map.grid[guard_position.y + 1][guard_position.x] == Entity::Empty {
                    map.grid[guard_position.y + 1][guard_position.x] = Entity::Obstruction;
                }
            }
        }
        Direction::West => {
            if guard_position.x != 0 {
                if map.grid[guard_position.y][guard_position.x - 1] == Entity::Empty {
                    map.grid[guard_position.y][guard_position.x - 1] = Entity::Obstruction;
                }
            }
        }
    }

    Ok(simulate_infinite_patrol(map, 3000))
}

fn simulate_infinite_patrol(mut map: Map, limit: i32) -> bool {
    info!("Simulating Infinite Patrol");

    let mut iterations = 0;
    while find_guard(&map).is_some() {
        if iterations > limit {
            return true;
        }
        map = progress_guard(map, true).expect("guard to progress");
        iterations += 1;
    }
    false
}

fn calculate_unique_cells(map: &Map) -> i32 {
    let mut unique_cells_count = 0;

    for row in map.get_grid() {
        for cell in row {
            if cell == &Entity::Path {
                unique_cells_count += 1;
            }
        }
    }

    unique_cells_count
}

#[derive(Debug, PartialEq)]
enum Day6Error {
    NoGuard,
}

impl fmt::Display for Day6Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Day6Error::NoGuard => write!(f, "no guard found in map"),
        }
    }
}

impl error::Error for Day6Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Day6Error::NoGuard => None,
        }
    }
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
    fn test_progress_guard() {
        test_init();
        let mut map = create_empty_map();
        map.grid[5][4] = Entity::Guard(Guard {
            direction: Direction::North,
        });

        let mut map_final = create_empty_map();
        map_final.grid[4][4] = Entity::Guard(Guard {
            direction: Direction::North,
        });
        map_final.grid[5][4] = Entity::Path;

        assert_eq!(
            map_final,
            progress_guard(map, true).expect("to receive map back"),
            "Expect the guard to be one space to the north"
        );
    }

    #[test]
    fn test_progress_guard_no_guard() {
        test_init();
        let map = create_empty_map();

        assert_eq!(
            Err(Day6Error::NoGuard),
            progress_guard(map, true),
            "Expect a Day6Error:NoGuard to be returned"
        );
    }

    #[test]
    fn test_progress_guard_out_of_bounds() {
        test_init();
        let mut map = create_empty_map();
        map.grid[0][4] = Entity::Guard(Guard {
            direction: Direction::North,
        });

        let mut map_final = create_empty_map();

        map_final.grid[0][4] = Entity::Path;

        assert_eq!(
            map_final,
            progress_guard(map, true).expect("to receive map back"),
            "Expected an empty map to be returned"
        );
    }

    #[test]
    fn test_progress_guard_obstacle() {
        test_init();
        let mut map = create_empty_map();
        map.grid[4][4] = Entity::Obstruction;
        map.grid[5][4] = Entity::Guard(Guard {
            direction: Direction::North,
        });

        let mut map_final = create_empty_map();
        map_final.grid[4][4] = Entity::Obstruction;
        map_final.grid[5][4] = Entity::Guard(Guard {
            direction: Direction::East,
        });

        debug!("Starting Map:\n{}", map);
        assert_eq!(
            map_final,
            progress_guard(map, true).expect("to receive map back"),
            "Expect the guard to turn to the east"
        );
    }

    #[test]
    fn test_simulate_patrol_sample_data() {
        test_init();

        assert_eq!(
            41,
            simulate_patrol(
                Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"),
                false
            )
            .expect("Simulation to work")
        )
    }

    #[test]
    fn test_simulate_patrol_add_obstructions_sample_data() {
        test_init();

        assert_eq!(
            6,
            simulate_patrol(
                Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"),
                true
            )
            .expect("Simulation to work")
        )
    }
}
