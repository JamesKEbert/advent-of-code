use std::{fs, io};

#[macro_use]
extern crate log;

fn main() {
    println!("Hello, world!");
}

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    info!("Reading File...");
    let contents = fs::read_to_string(file_path)?;
    info!("Read File!");
    trace!("File Contents: {:?}", contents);
    Ok(contents)
}

fn test_init() {
    env_logger::builder().is_test(true).try_init().ok();
}

pub mod day_1 {
    use std::collections::HashMap;

    use crate::read_file;

    fn parse_day_1_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
        info!("Parsing File");
        let content = read_file(file_path).expect("Content to be read");

        let mut left_list: Vec<i32> = vec![];
        let mut right_list: Vec<i32> = vec![];
        let split_content = content.split("\n");
        for line in split_content {
            let split_line = line.split("   ");
            for (index, value) in split_line.enumerate() {
                let num: i32 = value.parse().expect("To be a valid number");
                if index == 0 {
                    left_list.append(&mut vec![num]);
                } else {
                    right_list.append(&mut vec![num]);
                }
            }
        }
        debug!("Left List  '{:?}'", left_list);
        debug!("Right List '{:?}'", right_list);

        (left_list, right_list)
    }

    pub fn sort_list(short_first: bool, list: &mut Vec<i32>) -> Vec<i32> {
        info!("Sorting List...");

        // Suboptimal how with loop?
        loop {
            let mut swaps = 0;
            debug!("Sorting...");
            for index in 0..list.len() - 1 {
                let value = list[index];
                let next_value = list[index + 1];

                // debug!(
                //     "list[{}]:'{}', list[{}]:'{}'",
                //     index,
                //     value,
                //     index + 1,
                //     next_value
                // );

                if (value > next_value) && short_first {
                    list.swap(index, index + 1);
                    swaps += 1;
                }

                if (value < next_value) && !short_first {
                    list.swap(index, index + 1);
                    swaps += 1;
                }

                debug!("List '{:?}'", list);
            }
            if swaps == 0 {
                debug!("Sorted List {:?}", list);
                info!("Sorting Complete");
                break;
            }
        }

        list.clone()
    }

    pub fn tupilize(left_list: Vec<i32>, right_list: Vec<i32>) -> Vec<(i32, i32)> {
        info!("Tupalizing Lists");
        debug!("Left List  '{:?}'", left_list);
        debug!("Right List '{:?}'", right_list);
        let mut list: Vec<(i32, i32)> = vec![];
        for index in 0..left_list.len() {
            list.append(&mut vec![(left_list[index], right_list[index])]);
        }
        info!("Tupalized List {:?}", list);
        list
    }

    pub fn count_similarities(list: &mut Vec<i32>) -> HashMap<i32, i32> {
        info!("Counting similarities in List");
        let mut count_map: HashMap<i32, i32> = HashMap::new();

        for value in list {
            count_map
                .entry(value.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        debug!("Count Map: {:?}", count_map);
        info!("Counted similarities in list");
        count_map
    }

    pub fn calculate_distance(file_path: &str) -> i32 {
        info!("Beginning to calculate distance");

        let (left_list, right_list) = parse_day_1_file(file_path);

        debug!("Left List  '{:?}'", left_list);
        debug!("Right List '{:?}'", right_list);

        let tupalized_list = tupilize(
            sort_list(true, &mut left_list.clone()),
            sort_list(true, &mut right_list.clone()),
        );

        info!("Calculating Distance");
        let mut distance = 0;
        for tuple in tupalized_list {
            distance += (tuple.0 - tuple.1).abs();
        }

        info!("Calculated Distance: '{}'", distance);
        distance
    }

    pub fn calculate_score(file_path: &str) -> i32 {
        info!("Beginning to calculate score");

        let (left_list, right_list) = parse_day_1_file(file_path);

        debug!("Left List  '{:?}'", left_list);
        debug!("Right List '{:?}'", right_list);

        let sorted_left_list = sort_list(true, &mut left_list.clone());
        let count_map = count_similarities(&mut sort_list(true, &mut right_list.clone()));

        let mut score = 0;
        for value in sorted_left_list {
            score += value
                * count_map
                    .get(&value)
                    .or(Some(&0))
                    .expect("To be a valid int");
        }

        info!("Calculated Score: {}", score);
        score
    }

    #[cfg(test)]
    mod tests {
        use crate::test_init;

        use super::*;

        #[test]
        fn test_read_file() {
            test_init();
            assert_eq!(
                parse_day_1_file("./src/puzzle_inputs/day_1_sample.txt"),
                (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
            );
        }

        #[test]
        fn sort_array() {
            test_init();
            let sorted_list = sort_list(true, &mut vec![3, 4, 2, 1, 3, 3]);
            assert_eq!(sorted_list, vec![1, 2, 3, 3, 3, 4])
        }

        #[test]
        fn sort_array_reverse() {
            test_init();
            let sorted_list = sort_list(false, &mut vec![3, 4, 2, 1, 3, 3]);
            assert_eq!(sorted_list, vec![4, 3, 3, 3, 2, 1])
        }

        #[test]
        fn test_tupalize() {
            test_init();
            let tupalized_list = tupilize(vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
            assert_eq!(
                tupalized_list,
                vec![(1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9)]
            );
        }

        #[test]
        fn test_count_similarities() {
            test_init();
            let count_map = count_similarities(&mut vec![4, 3, 5, 3, 9, 3]);
            let mut correct_count_map = HashMap::new();
            correct_count_map.insert(3, 3);
            correct_count_map.insert(4, 1);
            correct_count_map.insert(5, 1);
            correct_count_map.insert(9, 1);
            assert_eq!(count_map, correct_count_map)
        }

        #[test]
        fn example_input() {
            test_init();
            let total_distance = calculate_distance("./src/puzzle_inputs/day_1_sample.txt");
            assert_eq!(total_distance, 11);
        }

        #[test]
        fn example_similarity_score() {
            test_init();
            let score = calculate_score("./src/puzzle_inputs/day_1_sample.txt");
            assert_eq!(score, 31);
        }
    }
}
