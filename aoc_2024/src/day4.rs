use camino::Utf8PathBuf;
use clap::Subcommand;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day4Commands {
    /// Searches Puzzle for XMAS/SAMX
    SearchPuzzle {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
}

pub fn day4_cli_command_processing(command: &Day4Commands) {
    match command {
        Day4Commands::SearchPuzzle { path } => {
            info!("Command received to search puzzle");
            println!("Total XMAS/SAMX: {}", search_puzzle(path.clone()));
        }
    }
}
type Puzzle = Vec<Line>;
type Line = Vec<char>;

fn parse_file(file_path: Utf8PathBuf) -> Puzzle {
    let mut puzzle: Puzzle = vec![];

    let contents = read_file(file_path);
    let lines = contents.split("\n");
    for line in lines {
        let vec_line: Line = line.chars().into_iter().collect();
        puzzle.append(&mut vec![vec_line]);
    }

    puzzle
}

fn search_iterator_for_xmas(line: Line) -> i32 {
    let mut count = 0;
    for (index, char) in line.iter().enumerate().clone() {
        if char == &'X'
            && index + 3 < line.len()
            && line[index + 1] == 'M'
            && line[index + 2] == 'A'
            && line[index + 3] == 'S'
        {
            count += 1
        }
    }
    debug!("Line '{:?}', count of XMAS '{}'", line, count);
    count
}

fn search_iterator_for_samx(line: Line) -> i32 {
    let mut count = 0;
    for (index, char) in line.iter().enumerate().clone() {
        if char == &'S'
            && index + 3 < line.len()
            && line[index + 1] == 'A'
            && line[index + 2] == 'M'
            && line[index + 3] == 'X'
        {
            count += 1
        }
    }
    debug!("Line '{:?}', count of SAMX '{}'", line, count);
    count
}

fn search_horizontally(puzzle: Puzzle) -> i32 {
    let mut count = 0;

    for line in puzzle {
        count += search_iterator_for_xmas(line.clone());
        count += search_iterator_for_samx(line);
    }

    count
}

fn search_vertically(puzzle: Puzzle) -> i32 {
    let mut count = 0;
    let mut index = 0;
    while index < puzzle[0].len() {
        let mut vertical_line: Vec<char> = vec![];
        for line in &puzzle {
            vertical_line.push(line[index]);
        }
        debug!("Vertical_Line: {:?}", vertical_line);
        count += search_iterator_for_xmas(vertical_line.clone());
        count += search_iterator_for_samx(vertical_line);
        index += 1;
    }

    count
}

// This is somewhat inefficient and I don't like, but I don't care to fix it right now. Bite me.
fn search_diagonally_right(puzzle: Puzzle) -> i32 {
    let mut count = 0;

    for (line_index, line) in puzzle.iter().enumerate() {
        for (char_index, _char) in line.iter().enumerate() {
            let mut x_index = char_index;
            let mut y_index = line_index;
            let mut diagonal_line: Vec<char> = vec![];
            while x_index < line.len()
                && y_index < puzzle.len()
                && x_index - char_index < 4
                && y_index - line_index < 4
            {
                // debug!("x_index {}, y_index {}", x_index, y_index);
                diagonal_line.push(puzzle[y_index][x_index]);
                x_index += 1;
                y_index += 1;
            }
            debug!("Diagonal Line: {:?}", diagonal_line);
            count += search_iterator_for_xmas(diagonal_line.clone());
            count += search_iterator_for_samx(diagonal_line);
        }
    }

    count
}

fn search_diagonally_left(puzzle: Puzzle) -> i32 {
    let mut count = 0;

    for (line_index, line) in puzzle.iter().enumerate() {
        for (char_index, _char) in line.iter().enumerate() {
            let mut x_index = char_index;
            let mut y_index = line_index;
            let mut diagonal_line: Vec<char> = vec![];
            'line: while y_index < puzzle.len()
                && char_index - x_index < 4
                && y_index - line_index < 4
            {
                // debug!("x_index {}, y_index {}", x_index, y_index);
                diagonal_line.push(puzzle[y_index][x_index]);
                if x_index == 0 {
                    break 'line;
                }
                x_index -= 1;
                y_index += 1;
            }
            debug!("Diagonal Line: {:?}", diagonal_line);
            count += search_iterator_for_xmas(diagonal_line.clone());
            count += search_iterator_for_samx(diagonal_line);
        }
    }

    count
}

fn search_diagonally(puzzle: Puzzle) -> i32 {
    let mut count = 0;
    count += search_diagonally_right(puzzle.clone());
    count += search_diagonally_left(puzzle);

    count
}

fn search_puzzle(file_path: Utf8PathBuf) -> i32 {
    let mut count = 0;
    let puzzle: Puzzle = parse_file(file_path);
    count += search_diagonally(puzzle.clone());
    count += search_horizontally(puzzle.clone());
    count += search_vertically(puzzle);
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    fn sample_puzzle_vectors() -> Puzzle {
        vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]
    }

    #[test]
    fn test_xmas_scan_sample() {
        test_init();
        assert_eq!(
            sample_puzzle_vectors(),
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day4_sample.txt"))
        );
    }

    #[test]
    fn test_xmas_horizontal_search() {
        test_init();
        assert_eq!(5, search_horizontally(sample_puzzle_vectors()))
    }

    #[test]
    fn test_xmas_vertical_search() {
        test_init();
        assert_eq!(3, search_vertically(sample_puzzle_vectors()))
    }

    #[test]
    fn test_xmas_diagonal_search_right() {
        test_init();
        assert_eq!(5, search_diagonally_right(sample_puzzle_vectors()))
    }
    #[test]
    fn test_xmas_diagonal_search_left() {
        test_init();
        assert_eq!(5, search_diagonally_left(sample_puzzle_vectors()))
    }
    #[test]
    fn test_xmas_diagonal_search() {
        test_init();
        assert_eq!(10, search_diagonally(sample_puzzle_vectors()))
    }

    #[test]
    fn test_puzzle_search_sample() {
        test_init();
        assert_eq!(
            18,
            search_puzzle(Utf8PathBuf::from("./src/puzzle_inputs/day4_sample.txt"))
        )
    }
}
