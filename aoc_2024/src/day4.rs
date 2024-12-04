use camino::Utf8PathBuf;
use regex::Regex;

use crate::read_file;

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
}
