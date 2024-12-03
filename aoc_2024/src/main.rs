use std::fs;

#[macro_use]
extern crate log;

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use day_1::{calculate_distance, calculate_score};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify level of logs emitted
    #[arg(long, default_value_t = log::LevelFilter::Info)]
    loglevel: log::LevelFilter,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run Day1 methods against input files
    Day1 {
        #[command(subcommand)]
        command: Day1Commands,
    },
}

#[derive(Subcommand, Debug)]
enum Day1Commands {
    /// Calculate the Total Distance from the two lists
    TotalDistance {
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
    /// Calculate the Similarity Score from two lists
    Score {
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut builder = colog::default_builder();
    builder.filter(None, cli.loglevel);
    builder.init();

    match &cli.command {
        Commands::Day1 { command } => match command {
            Day1Commands::TotalDistance { path } => {
                info!("Command received to calculate Total Distance");
                println!("Total Distance: {}", calculate_distance(path.clone()));
            }
            Day1Commands::Score { path } => {
                info!("Command received to calculate Similarity Score");
                println!("Total Similarity Score: {}", calculate_score(path.clone()));
            }
        },
    }
}

pub fn read_file(file_path: Utf8PathBuf) -> String {
    info!("Reading File...");
    // Using expect here, not doing file validation. If the process fails here, we'll consider that a user error. Obviously validation/handling would be ideal, but I don't care in this context.
    let contents = fs::read_to_string(file_path).expect("Content to be parsed correctly");
    info!("Read File!");
    trace!("File Contents: {:?}", contents);
    contents
}

#[cfg(test)]
fn test_init() {
    env_logger::builder().is_test(true).try_init().ok();
}

pub mod day_1 {
    use std::collections::HashMap;

    use camino::Utf8PathBuf;

    use crate::read_file;

    fn parse_day_1_file(file_path: Utf8PathBuf) -> (Vec<i32>, Vec<i32>) {
        info!("Parsing File");
        let content = read_file(file_path);

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
        info!("Tupalized List");
        debug!("List: {:?}", list);
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

    pub fn calculate_distance(file_path: Utf8PathBuf) -> i32 {
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

    pub fn calculate_score(file_path: Utf8PathBuf) -> i32 {
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
        use camino::Utf8PathBuf;

        use crate::test_init;

        use super::*;

        #[test]
        fn test_read_file() {
            test_init();
            assert_eq!(
                parse_day_1_file(Utf8PathBuf::from("./src/puzzle_inputs/day_1_sample.txt")),
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
            let total_distance =
                calculate_distance(Utf8PathBuf::from("./src/puzzle_inputs/day_1_sample.txt"));
            assert_eq!(total_distance, 11);
        }

        #[test]
        fn example_similarity_score() {
            test_init();
            let score = calculate_score(Utf8PathBuf::from("./src/puzzle_inputs/day_1_sample.txt"));
            assert_eq!(score, 31);
        }
    }
}

mod day_2 {
    use camino::Utf8PathBuf;

    use crate::read_file;

    type Report = Vec<Level>;
    type Level = i32;

    pub fn parse_file(file_path: Utf8PathBuf) -> Vec<Report> {
        info!("Parsing File");
        let content = read_file(file_path);
        let reports_string = content.split("\n");

        let mut reports: Vec<Report> = vec![];
        for report_string in reports_string {
            let levels_string = report_string.split(" ");
            let levels: Vec<Level> = levels_string
                .map(|level| level.parse::<i32>().expect("To be a valid number"))
                .collect();
            reports.append(&mut vec![levels]);
        }
        reports
    }

    #[derive(PartialEq)]
    enum Direction {
        Decreasing,
        Increasing,
        NotSet,
    }

    pub fn validate_increasing_or_decreasing(report: Report) -> bool {
        info!("Validating Increasing or Decreasing Levels in Report");
        debug!("Report {:?}", report);
        let mut direction = Direction::NotSet;
        let mut last_level = report[0];
        for level in &report[1..] {
            if direction == Direction::NotSet {
                if &last_level < level {
                    direction = Direction::Increasing;
                    debug!("Report Determined to be increasing");
                } else if &last_level > level {
                    direction = Direction::Decreasing;
                    debug!("Report Determined to be decreasing");
                }
            }

            if direction == Direction::Increasing && level < &last_level {
                info!("Report failed validation - report not consistently increasing");
                return false;
            }
            if level == &last_level {
                info!("Report failed validation - report level did not change between levels");
                return false;
            }
            if direction == Direction::Decreasing && level > &last_level {
                info!("Report failed validation - report not consistently decreasing");
                return false;
            }

            last_level = level.clone();
        }
        info!("Reported validated true");
        true
    }

    pub fn validate_adjacency_difference(report: Report) -> bool {
        info!("Validating Adjacency rules in report");
        debug!("Report {:?}", report);
        for (slice_index, level) in report[1..report.len() - 1].iter().enumerate() {
            let index = slice_index + 1;
            let backwards_difference = (level - report[index - 1]).abs();
            let forwards_difference = (level - report[index + 1]).abs();

            debug!(
                "Levels - {} {} {}; backwards_difference: {}, forwards_difference: {}",
                report[index - 1],
                level,
                report[index + 1],
                backwards_difference,
                forwards_difference
            );
            if backwards_difference > 3 {
                info!("Report failed validation - levels exceeded difference");
                return false;
            }
            if forwards_difference > 3 {
                info!("Report failed validation - levels exceeded difference");
                return false;
            }
        }
        info!("Report validated true");
        true
    }

    pub fn validate_report(report: Report) -> bool {
        if validate_increasing_or_decreasing(report.clone())
            && validate_adjacency_difference(report)
        {
            return true;
        }
        return false;
    }

    pub fn count_safe_reports(file_path: Utf8PathBuf) -> i32 {
        let reports: Vec<Report> = parse_file(file_path);
        let mut count = 0;

        for report in reports {
            if validate_report(report) {
                count += 1;
            }
        }

        count
    }

    #[cfg(test)]
    mod tests {
        use crate::test_init;

        use super::*;

        #[test]
        fn test_input_parsing() {
            test_init();
            assert_eq!(
                parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day_2_sample.txt")),
                vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9]
                ]
            )
        }

        #[test]
        fn test_decreasing_report_success() {
            test_init();
            assert!(validate_report(vec![7, 6, 4, 2, 1]))
        }

        #[test]
        fn test_large_level_increase_failure() {
            test_init();
            assert_eq!(false, validate_report(vec![1, 2, 7, 8, 9]))
        }

        #[test]
        fn test_large_level_decrease_failure() {
            test_init();
            assert_eq!(false, validate_report(vec![9, 7, 6, 2, 1]))
        }

        #[test]
        fn test_direction_switch_failure() {
            test_init();
            assert_eq!(false, validate_report(vec![1, 3, 2, 4, 5]))
        }

        #[test]
        fn test_levels_stable_failure() {
            test_init();
            assert_eq!(false, validate_report(vec![8, 6, 4, 4, 1],))
        }

        #[test]
        fn test_increasing_report_success() {
            test_init();
            assert!(validate_report(vec![1, 3, 6, 7, 9]))
        }

        // Not mentioned in AoC, but I'm testing anyways
        #[test]
        fn test_decreasing_report_false() {
            test_init();
            assert_eq!(false, validate_report(vec![7, 6, 4, 2, 3]))
        }

        // Not mentioned in AoC, but I'm testing anyways
        #[test]
        fn test_adjacency_levels_true() {
            test_init();
            assert!(validate_report(vec![1, 3, 6, 7, 9]))
        }

        #[test]
        fn test_sample_count() {
            test_init();
            assert_eq!(
                2,
                count_safe_reports(Utf8PathBuf::from("./src/puzzle_inputs/day_2_sample.txt"))
            )
        }
    }
}
