use camino::Utf8PathBuf;

use std::{
    fmt::{self, Debug, Display},
    ops::Index,
    vec,
};

#[derive(Clone, PartialEq, Debug)]
enum Entity {
    Empty,
    Obstruction,
    Guard(Guard),
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
        }
    }
}

// impl Debug for Entity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug("Point")
//             .field("x", &self.x)
//             .field("y", &self.y)
//             .finish()
//     }
// }

struct Map {
    grid: Vec<Vec<Entity>>,
}
impl Map {
    pub fn new(grid: Vec<Vec<Entity>>) -> Self {
        Map { grid }
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

fn parse_file(file_path: Utf8PathBuf) -> Map {
    let mut map: Map = Map::new(vec![]);

    // for file

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    #[test]
    fn test_file_input() {
        test_init();
        let mut map: Map = Map::new(vec![
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

        debug!("Map: \n{}", map);

        // assert_eq!(
        //     map,
        //     parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day6_sample.txt"))
        // )
    }
}
