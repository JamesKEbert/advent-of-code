use std::usize;

use camino::Utf8PathBuf;
use clap::Subcommand;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day9Commands {
    /// Calculates checksum
    CalculateChecksum {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
}

pub fn day9_cli_command_processing(command: &Day9Commands) {
    match command {
        Day9Commands::CalculateChecksum { path } => {
            info!("Command received to calculate disk checksum");
            println!("checksum: {}", calculate_file_checksum(path.clone()));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
    File(usize),
    Empty,
}

fn parse_disk_map(disk: &str) -> Vec<Block> {
    debug!("Parsing Disk Map '{}'", disk);
    let mut parsed_disk = vec![];
    let mut file_indicator = true;
    for (index, char) in disk.chars().enumerate() {
        trace!(
            "Index {}, Char: {}, file_indicator: {}",
            index,
            char,
            file_indicator
        );
        if file_indicator {
            // parsed_disk.append(&mut vec![char::from_digit(Some(index /2).filter(|size| size != &(0)).unwrap_or(1) as u32, 10).expect("To be valid number"); char.to_digit(10).expect("Valid Numbers") as usize]);
            parsed_disk.append(&mut vec![
                Block::File(index / 2);
                char.to_digit(10).expect("Valid Numbers") as usize
            ]);
        } else {
            parsed_disk.append(&mut vec![
                Block::Empty;
                char.to_digit(10).expect("Valid Numbers") as usize
            ]);
        }
        file_indicator = !file_indicator;
        trace!("Partially Parsed Disk Map: '{:?}'", parsed_disk);
    }
    debug!("Parsed Disk Map: '{:?}'", parsed_disk);
    parsed_disk
}

fn is_sorted(disk: &Vec<Block>) -> bool {
    let mut found_dot = false;
    for block in disk {
        if block == &Block::Empty {
            found_dot = true;
        } else {
            if found_dot {
                return false;
            }
        }
    }

    true
}

fn sort_disk_map(disk: Vec<Block>) -> Vec<Block> {
    info!("Sorting disk map: '{:?}'", disk);
    let mut sorted_disk = disk.clone();

    let mut iterator = 0;

    for (index, block) in disk.iter().enumerate() {
        info!("Index: {}", index);
        if is_sorted(&sorted_disk) {
            break;
        }
        if block == &Block::Empty {
            let (reverse_index, _block) = sorted_disk
                .iter()
                .rev()
                .enumerate()
                .find(|(_i, block)| block != &&Block::Empty)
                .expect("block to exist");

            let index_b = sorted_disk.len() - 1 - reverse_index;
            trace!("Indexes to swap '{}', '{}'", index, index_b);
            sorted_disk.swap(index, index_b);
        }
        trace!("Partially sorted disk map: '{:?}'", sorted_disk);
    }

    info!("Sorted disk map: '{:?}'", sorted_disk);
    sorted_disk
}

fn calculate_file_checksum(filepath: Utf8PathBuf) -> u64 {
    let content = read_file(filepath);
    let sorted_disk = sort_disk_map(parse_disk_map(&content));

    let mut checksum = 0;
    for (index, block) in sorted_disk.iter().enumerate() {
        match block {
            Block::File(usize) => checksum += (usize * index) as u64,
            Block::Empty => (),
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use super::*;
    use crate::test_init;

    #[test]
    fn test_parse_disk_map_simple() {
        test_init();
        assert_eq!(
            vec![
                Block::File(0),
                Block::Empty,
                Block::Empty,
                Block::File(1),
                Block::File(1),
                Block::File(1),
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::File(2)
            ],
            parse_disk_map("12345")
        )
    }

    #[test]
    fn test_sort_disk_map_simple() {
        test_init();
        assert_eq!(
            vec![
                Block::File(0),
                Block::File(2),
                Block::File(2),
                Block::File(1),
                Block::File(1),
                Block::File(1),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::Empty,
            ],
            sort_disk_map(parse_disk_map("12345"))
        )
    }

    #[test]
    fn test_sample_file() {
        test_init();
        assert_eq!(
            1928,
            calculate_file_checksum(Utf8PathBuf::from("./src/puzzle_inputs/day9_sample.txt"))
        )
    }
}
