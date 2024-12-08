use camino::Utf8PathBuf;

use clap::Subcommand;
use std::{
    collections::HashMap,
    error,
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
        /// Patrol Simulation Limit
        #[arg(long, default_value_t = 10000)]
        limit: i32,
    },
    /// Calculates total number of valid positions for obstructions that would create an infinite loop
    CheckObstructions {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
        /// Patrol Simulation Limit
        #[arg(long, default_value_t = 10000)]
        limit: i32,
    },
}

pub fn day6_cli_command_processing(command: &Day6Commands) {
    match command {
        Day6Commands::Calculate { path, limit } => {
            info!("Command received to calculate total distinct cells for guard");
            println!(
                "Total Number of Distinct Cells for guard's path: {}",
                count_distinct_cells(path.clone(), limit.to_owned())
            );
        }
        Day6Commands::CheckObstructions { path, limit } => {
            info!("Command received to check number of valid obstructions");
            println!(
                "Total Number of valid obstruction positions: {}",
                test_obstructions(path.clone(), limit.to_owned())
            );
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Entity {
    Empty,
    Obstruction,
    Guard(Guard),
    Path(Path),
    TempObstruction,
}

#[derive(Clone, PartialEq, Debug)]
enum Path {
    Vertical,
    Horizontal,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
struct Guard {
    direction: Direction,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
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
            Entity::Path(path) => match path {
                Path::Vertical => write!(f, "|"),
                Path::Horizontal => write!(f, "-"),
            },
            Entity::TempObstruction => write!(f, "O"),
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

#[derive(PartialEq, Eq, Hash, Debug)]
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
    // NoGuard,
    // GoingOutOfBounds,
    ExceededLimit,
    InfiniteLoop,
}

impl fmt::Display for Day6Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Day6Error::NoGuard => write!(f, "no guard found in map"),
            // Day6Error::GoingOutOfBounds => write!(f, "guard going out of bounds"),
            Day6Error::ExceededLimit => {
                write!(f, "guard simulation exceeded simulation iteration limit")
            }
            Day6Error::InfiniteLoop => {
                write!(f, "guard simulation is in an infinite loop")
            }
        }
    }
}

impl error::Error for Day6Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // Day6Error::NoGuard => None,
            // Day6Error::GoingOutOfBounds => None,
            Day6Error::ExceededLimit => None,
            Day6Error::InfiniteLoop => None,
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
    for row in &map.grid {
        for cell in row {
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

fn move_guard(mut map: Map, guard: &mut Guard) -> (Map, Guard) {
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

fn simulate_patrol(
    mut map: Map,
    guard: &mut Guard,
    limit: i32,
    detect_loop: bool,
) -> Result<HashMap<Guard, Guard>, Day6Error> {
    trace!("Simulating Patrol");
    let mut guard_positions: HashMap<Guard, Guard> = HashMap::new();

    let mut iteration = 0;
    loop {
        trace!("Iteration {}", iteration);
        if iteration > limit {
            debug!("Simulation hit limit set");
            return Err(Day6Error::ExceededLimit);
        }
        if guard.x == usize::MAX || guard.y == usize::MAX {
            break;
        }

        if detect_loop {
            if guard_positions.contains_key(&guard.clone()) {
                trace!("Detected Loop!");
                return Err(Day6Error::InfiniteLoop);
            }
        }
        guard_positions.insert(guard.clone(), guard.clone());
        (map, *guard) = move_guard(map, guard);
        iteration += 1;
    }

    Ok(guard_positions)
}

fn count_distinct_cells(file_path: Utf8PathBuf, simulation_limit: i32) -> usize {
    let map = parse_file(file_path);
    let mut guard = find_guard(&map).expect("guard to exist in map");
    let guard_positions = simulate_patrol(map, &mut guard, simulation_limit, false)
        .expect("expected map to be simulated correctly");

    let mut unique_positions: HashMap<Position, Guard> = HashMap::new();
    for (_unique_position, unique_guard) in guard_positions {
        unique_positions.insert(
            Position {
                x: unique_guard.x,
                y: unique_guard.y,
            },
            unique_guard,
        );
    }
    unique_positions.len()
}

fn test_obstructions(file_path: Utf8PathBuf, simulation_limit: i32) -> usize {
    let map = parse_file(file_path);
    let mut guard = find_guard(&map).expect("guard to exist in map");
    let obstruction_guard = guard.clone();
    let guard_positions = simulate_patrol(map.clone(), &mut guard, simulation_limit, false)
        .expect("expected map to be simulated correctly");

    // Pretty print the guard's path
    let mut patrol_path_map = map.clone();
    for (_position, historical_guard) in guard_positions.clone() {
        match historical_guard.direction {
            Direction::North | Direction::South => {
                patrol_path_map.grid[historical_guard.y][historical_guard.x] =
                    Entity::Path(Path::Vertical);
            }
            Direction::East | Direction::West => {
                patrol_path_map.grid[historical_guard.y][historical_guard.x] =
                    Entity::Path(Path::Horizontal);
            }
        }
    }
    info!("Patrol Path:\n{}", patrol_path_map);

    let mut unique_positions: HashMap<Position, Guard> = HashMap::new();
    for (_unique_position, unique_guard) in guard_positions {
        unique_positions.insert(
            Position {
                x: unique_guard.x,
                y: unique_guard.y,
            },
            unique_guard,
        );
    }

    let mut valid_obstruction_count = 0;

    let mut iteration = 0;
    let total_iterations = unique_positions.len();
    for (_position, historical_guard) in unique_positions {
        info!(
            "Obstruction Position Check {}/{}",
            iteration, total_iterations
        );
        if !(historical_guard.x == obstruction_guard.x && historical_guard.y == obstruction_guard.y)
        {
            let mut test_map = map.clone();
            test_map.grid[historical_guard.y][historical_guard.x] = Entity::Obstruction;

            let mut display_test_map = map.clone();
            display_test_map.grid[historical_guard.y][historical_guard.x] = Entity::TempObstruction;
            trace!("Testing map:\n{}", display_test_map);

            if simulate_patrol(
                test_map.clone(),
                &mut obstruction_guard.clone(),
                simulation_limit,
                true,
            )
            .is_err()
            {
                debug!("Valid Obstruction");
                valid_obstruction_count += 1;
            } else {
                debug!("Invalid Obstruction");
            }
        } else {
            info!(
                "Filtering out starting guard location {}, present coordinates {}",
                Position {
                    x: obstruction_guard.x,
                    y: obstruction_guard.y
                },
                Position {
                    x: historical_guard.x,
                    y: historical_guard.y
                },
            );
        }
        iteration += 1;
    }

    valid_obstruction_count
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

        let mut guard_positions: HashMap<Guard, Guard> = HashMap::new();
        let guard1 = Guard {
            x: 4,
            y: 2,
            direction: Direction::North,
        };
        guard_positions.insert(guard1.clone(), guard1);

        let guard2 = Guard {
            x: 4,
            y: 1,
            direction: Direction::North,
        };
        guard_positions.insert(guard2.clone(), guard2);

        let guard3 = Guard {
            x: 4,
            y: 0,
            direction: Direction::North,
        };
        guard_positions.insert(guard3.clone(), guard3);

        assert_eq!(
            guard_positions,
            simulate_patrol(map, &mut original_guard, 100, false)
                .expect("to be simulated correctly"),
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
                100
            )
        )
    }

    #[test]
    fn test_add_obstructions_sample_data() {
        test_init();

        assert_eq!(
            6,
            test_obstructions(
                Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"),
                1000
            )
        )
    }
}
