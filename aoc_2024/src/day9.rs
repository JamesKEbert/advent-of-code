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
            println!(
                "checksum: {}",
                calculate_file_checksum(path.clone())
            );
        }
    }
}

fn parse_disk_map(disk: &str) -> Vec<char>{
    info!("Parsing Disk Map '{}'", disk);
    let mut parsed_disk = vec![];
    let mut file_indicator = true;
    for (index, char) in disk.chars().enumerate(){
        trace!("Index {}, Char: {}, file_indicator: {}", index, char, file_indicator);
        if file_indicator {
            // parsed_disk.append(&mut vec![char::from_digit(Some(index /2).filter(|size| size != &(0)).unwrap_or(1) as u32, 10).expect("To be valid number"); char.to_digit(10).expect("Valid Numbers") as usize]);
            parsed_disk.append(&mut vec![char::from_digit((index / 2) as u32, 10).expect("to be a valid character"); char.to_digit(10).expect("Valid Numbers") as usize]);
        }else{
            parsed_disk.append(&mut vec!['.'; char.to_digit(10).expect("Valid Numbers") as usize]);
        }
        file_indicator = !file_indicator;
        trace!("Partially Parsed Disk Map: '{:?}'", parsed_disk);
    }
    info!("Parsed Disk Map: '{:?}'", parsed_disk);
    parsed_disk
}

fn is_sorted(disk: &Vec<char>) -> bool {
    let mut found_dot = false;
    for char in disk {
        if char == &'.' {
            found_dot = true;
        } else {
            if found_dot {
                return false
            }
        }
    }

    true
}

fn sort_disk_map(disk: Vec<char>) -> Vec<char> {
    info!("Sorting disk map: '{:?}'", disk);
    let mut sorted_disk = disk.clone();

    for (index, char) in disk.iter().enumerate() {
        if is_sorted(&sorted_disk) {
            break;
        }
        if char == &'.'{
            let (reverse_index, _char) = sorted_disk.iter().rev().enumerate().find(|(_i, char)| char != &&'.').expect("number to exist");

            let index_b = sorted_disk.len() - 1 - reverse_index;
            trace!("Indexes to swap '{}', '{}'", index, index_b);
            sorted_disk.swap(index, index_b);
        }
            info!("Partially sorted disk map: '{:?}'", sorted_disk);
    }

    info!("Sorted disk map: '{:?}'", sorted_disk);
    sorted_disk
}

fn calculate_file_checksum(filepath: Utf8PathBuf) -> u32 {
    let content = read_file(filepath);
    let sorted_disk = sort_disk_map(parse_disk_map(&content));

    let mut checksum = 0;
    for (index, char) in sorted_disk.iter().enumerate(){
        if char != &'.'{
            checksum += char.to_digit(10).expect("Valid Numbers") * index as u32;
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
    fn test_parse_disk_map_simple(){
        test_init();
        assert_eq!(vec!['0','.','.','1','1','1','.','.','.','.','2','2','2','2','2'], parse_disk_map("12345"))
    }

    #[test]
    fn test_sort_disk_map_simple(){
        test_init();
        assert_eq!(vec!['0','2','2','1','1','1','2','2','2','.','.','.','.','.','.'], sort_disk_map(parse_disk_map("12345")))
    }

    #[test]
    fn test_sample_file(){
        test_init();
        assert_eq!(1928, calculate_file_checksum(Utf8PathBuf::from("./src/puzzle_inputs/day9_sample.txt")))
    }
}