use std::{fmt::Display, usize};

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

struct Block2Vec(Vec<Block2>);
impl Display for Block2Vec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.0.iter() {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Block2 {
    File(File),
    Empty(Empty),
}

impl Display for Block2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block2::File(file) => {
                for _ in 0..file.block_length {
                    write!(f, "{}", file.index)?;
                }
                Ok(())
            }
            Block2::Empty(empty) => {
                for _ in 0..empty.block_length {
                    write!(f, ".")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    index: usize,
    block_length: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Empty {
    block_length: usize,
}

fn parse_disk_map_block2(disk: &str) -> Vec<Block2> {
    debug!("Parsing Disk Map into Block2 format: '{}'", disk);
    let mut parsed_disk = vec![];

    let mut file_indicator = true;
    let mut file_index = 0;
    for (index, char) in disk.char_indices() {
        let block_length = char.to_digit(10).expect("Valid Numbers") as usize;
        if file_indicator {
            parsed_disk.push(Block2::File(File {
                index: file_index,
                block_length,
            }));
            file_index += 1;
        } else {
            parsed_disk.push(Block2::Empty(Empty { block_length }));
            // parsed_disk.append(&mut vec![Block2::Empty; block_length]);
        }
        file_indicator = !file_indicator;
    }

    debug!("Parsed Disk Map Block2 format: '{:?}'", parsed_disk);
    parsed_disk
}

fn parse_disk_map(disk: &str) -> Vec<Block> {
    debug!("Parsing Disk Map '{}'", disk);
    let mut parsed_disk = vec![];
    let mut file_indicator = true;
    for (index, char) in disk.char_indices() {
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

fn sort_disk_map_block2(disk: Vec<Block2>) -> Vec<Block2> {
    info!("Sorting disk Block2 map: '{}'", Block2Vec(disk.clone()));
    let mut sorted_disk = disk.clone();

    // Iterate from back to front
    for (index, block) in disk.iter().rev().enumerate() {
        match block {
            Block2::File(file) => {
                // for (empty_index, empty_block) in
                //     sorted_disk[0..(disk.len() - 1 - index)].iter().enumerate()
                // {

                // }

                let empty_option = sorted_disk[0..(disk.len() - 1 - index)]
                    .iter()
                    .enumerate()
                    .find(|(_i, inner_block)| match inner_block {
                        Block2::Empty(empty) => return empty.block_length >= file.block_length,
                        _ => false,
                    });

                if empty_option.is_some() {
                    let (empty_index, empty_block) = empty_option.expect("To be valid empty");
                    trace!(
                        "Empty Index '{}', empty block '{}', File Index '{}', file block '{}'",
                        empty_index,
                        empty_block,
                        index,
                        block
                    );

                    match empty_block {
                        Block2::Empty(empty) => {
                            if empty.block_length > file.block_length {
                                sorted_disk[empty_index] = Block2::Empty(Empty {
                                    block_length: empty.block_length - file.block_length,
                                });
                                sorted_disk.insert(empty_index, block.clone());
                            } else if empty.block_length == file.block_length {
                                sorted_disk[empty_index] = block.clone();
                            } else {
                                // Shouldn't be possible due to filter
                                panic!("Empty Block length less than file block length");
                            }
                            // Replace moved file with empty
                            sorted_disk[disk.len() - index] = Block2::Empty(Empty {
                                block_length: file.block_length,
                            });
                        }
                        _ => panic!("Unexpected File Block2!"),
                    }
                }

                // // Iterate from front to back
                // for (front_index, front_block) in
                //     sorted_disk[0..(disk.len() - 1 - index)].iter().enumerate()
                // {
                //     match front_block {
                //         Block2::Empty(empty_block) => {
                //             if empty_block.block_length > file_block.block_length {
                //                 sorted_disk.insert(front_index, block.clone());
                //             } else if empty_block.block_length == file_block.block_length {
                //                 sorted_disk.insert(front_index, block.clone());
                //             }
                //         }
                //         Block2::File(_) => (),
                //     }
                // }
            }
            Block2::Empty(_) => (),
        }
        trace!(
            "Partially sorted disk block2 map: '{}'",
            Block2Vec(sorted_disk.clone())
        );
    }

    info!(
        "Sorted disk Block2 map: '{}'",
        Block2Vec(sorted_disk.clone())
    );
    sorted_disk
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

    #[test]
    fn test_parse_block2_disk() {
        test_init();
        assert_eq!(
            vec![
                Block2::File(File {
                    index: 0,
                    block_length: 1
                }),
                Block2::Empty(Empty { block_length: 2 }),
                // Block2::Empty,
                Block2::File(File {
                    index: 1,
                    block_length: 3
                }),
                Block2::Empty(Empty { block_length: 4 }),
                // Block2::Empty,
                // Block2::Empty,
                // Block2::Empty,
                Block2::File(File {
                    index: 2,
                    block_length: 5
                }),
            ],
            parse_disk_map_block2("12345")
        )
    }

    #[test]
    fn test_sort_disk_map_block2_simple() {
        test_init();
        let sample_sorted_map = vec![
            Block2::File(File {
                index: 0,
                block_length: 2,
            }),
            Block2::File(File {
                index: 9,
                block_length: 2,
            }),
            Block2::File(File {
                index: 2,
                block_length: 1,
            }),
            Block2::File(File {
                index: 1,
                block_length: 3,
            }),
            Block2::File(File {
                index: 7,
                block_length: 3,
            }),
            Block2::Empty(Empty { block_length: 1 }),
            Block2::File(File {
                index: 4,
                block_length: 2,
            }),
            Block2::Empty(Empty { block_length: 1 }),
            Block2::File(File {
                index: 3,
                block_length: 3,
            }),
            Block2::Empty(Empty { block_length: 4 }),
            Block2::File(File {
                index: 5,
                block_length: 4,
            }),
            Block2::Empty(Empty { block_length: 1 }),
            Block2::File(File {
                index: 6,
                block_length: 4,
            }),
            Block2::Empty(Empty { block_length: 5 }),
            Block2::File(File {
                index: 8,
                block_length: 4,
            }),
            Block2::Empty(Empty { block_length: 2 }),
        ];
        debug!(
            "Sample Sorted Map: '{}'",
            Block2Vec(sample_sorted_map.clone())
        );
        assert_eq!(
            sample_sorted_map,
            sort_disk_map_block2(parse_disk_map_block2("2333133121414131402"))
        ) //00992111777.44.333....5555.6666.....8888..
    }
}
