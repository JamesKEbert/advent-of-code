use camino::Utf8PathBuf;
use clap::Subcommand;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day2Commands {
    /// Counts total number of safe reports
    Count {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
        /// Whether to use the dampener
        #[arg(long, default_value_t = false)]
        dampener: bool,
    },
}

pub fn day2_cli_command_processing(command: &Day2Commands) {
    match command {
        Day2Commands::Count { path, dampener } => {
            info!("Command received to count number of safe reports");
            println!(
                "Total Safe Reports: {}",
                count_safe_reports(path.clone(), dampener.clone())
            );
        }
    }
}

type Report = Vec<Level>;
type Level = i32;

fn parse_file(file_path: Utf8PathBuf) -> Vec<Report> {
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

fn validate_increasing_or_decreasing(report: Report) -> bool {
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

fn validate_adjacency_difference(report: Report) -> bool {
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

fn validate_report(report: Report, dampener: bool) -> bool {
    if validate_increasing_or_decreasing(report.clone())
        && validate_adjacency_difference(report.clone())
    {
        return true;
    }

    if dampener == true {
        info!("Dampener Set, attempting to remove levels to achieve successful report");

        for (index, _level) in report.iter().enumerate() {
            let mut altered_report = report.clone();
            altered_report.remove(index);
            if validate_increasing_or_decreasing(altered_report.clone())
                && validate_adjacency_difference(altered_report)
            {
                return true;
            }
        }
    }
    return false;
}

fn count_safe_reports(file_path: Utf8PathBuf, dampener: bool) -> i32 {
    let reports: Vec<Report> = parse_file(file_path);
    let mut count = 0;

    for report in reports {
        if validate_report(report, dampener) {
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
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day2_sample.txt")),
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
        assert!(validate_report(vec![7, 6, 4, 2, 1], false))
    }

    #[test]
    fn test_large_level_increase_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![1, 2, 7, 8, 9], false))
    }

    #[test]
    fn test_large_level_decrease_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![9, 7, 6, 2, 1], false))
    }

    #[test]
    fn test_direction_switch_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![1, 3, 2, 4, 5], false))
    }

    #[test]
    fn test_levels_stable_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![8, 6, 4, 4, 1], false))
    }

    #[test]
    fn test_increasing_report_success() {
        test_init();
        assert!(validate_report(vec![1, 3, 6, 7, 9], false))
    }

    // Not mentioned in AoC, but I'm testing anyways
    #[test]
    fn test_decreasing_report_false() {
        test_init();
        assert_eq!(false, validate_report(vec![7, 6, 4, 2, 3], false))
    }

    // Not mentioned in AoC, but I'm testing anyways
    #[test]
    fn test_adjacency_levels_true() {
        test_init();
        assert!(validate_report(vec![1, 3, 6, 7, 9], false))
    }

    #[test]
    fn test_sample_count() {
        test_init();
        assert_eq!(
            2,
            count_safe_reports(
                Utf8PathBuf::from("./src/puzzle_inputs/day2_sample.txt"),
                false
            )
        )
    }

    #[test]
    fn test_dampener_decreasing_report_success() {
        test_init();
        assert!(validate_report(vec![7, 6, 4, 2, 1], true))
    }

    #[test]
    fn test_dampener_large_level_increase_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![1, 2, 7, 8, 9], true))
    }

    #[test]
    fn test_dampener_large_level_decrease_failure() {
        test_init();
        assert_eq!(false, validate_report(vec![9, 7, 6, 2, 1], true))
    }

    #[test]
    fn test_dampener_direction_switch_success() {
        test_init();
        assert!(validate_report(vec![1, 3, 2, 4, 5], true))
    }

    #[test]
    fn test_dampener_levels_stable_success() {
        test_init();
        assert!(validate_report(vec![8, 6, 4, 4, 1], true))
    }

    #[test]
    fn test_dampener_increasing_report_success() {
        test_init();
        assert!(validate_report(vec![1, 3, 6, 7, 9], true))
    }

    #[test]
    fn test_sample_count_dampener() {
        test_init();
        assert_eq!(
            4,
            count_safe_reports(
                Utf8PathBuf::from("./src/puzzle_inputs/day2_sample.txt"),
                true
            )
        )
    }
}
