use camino::Utf8PathBuf;
use clap::Subcommand;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day5Commands {
    /// Calculates Valid Updates total of middle pages
    CalculateMiddlePages {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
}

pub fn day5_cli_command_processing(command: &Day5Commands) {
    match command {
        Day5Commands::CalculateMiddlePages { path } => {
            info!("Command received to calculate middle pages total");
            println!(
                "Total Number of Middle Pages from valid updates: {}",
                calculate_middle_page_total(path.clone())
            );
        }
    }
}

type Update = Vec<i32>;
type Rule = Vec<i32>;

fn parse_file(file_path: Utf8PathBuf) -> (Vec<Rule>, Vec<Update>) {
    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Update> = vec![];

    let content = read_file(file_path);
    let split_content: Vec<&str> = content.split("\n\n").collect();

    let rules_strings: Vec<&str> = split_content[0].split("\n").collect();
    for rule_content in rules_strings {
        let rule: Rule = rule_content
            .split("|")
            .map(|level| level.parse::<i32>().expect("To be a valid number"))
            .collect();
        rules.push(rule);
    }

    let updates_strings: Vec<&str> = split_content[1].split("\n").collect();
    for update_content in updates_strings {
        let update: Update = update_content
            .split(",")
            .map(|level| level.parse::<i32>().expect("To be a valid number"))
            .collect();
        updates.push(update);
    }

    (rules, updates)
}

fn validate_update_order(update: &Update, rules: &Vec<Rule>) -> bool {
    'rule: for rule in rules {
        for page_num in update.clone() {
            if update.contains(&rule[0]) && update.contains(&rule[1]) {
                trace!(
                    "Update contains Page Numbers corresponding to Rule '{},{}'",
                    rule[0],
                    rule[1]
                );
                // If we get to the second page number in the rule before the first one, then the update is invalid
                if page_num == rule[1] {
                    trace!("Update page numbers are incorrectly ordered, update is invalid");
                    return false;
                }
                if page_num == rule[0] {
                    trace!("Update is valid for Rule '{},{}'", rule[0], rule[1]);
                    continue 'rule;
                }
            }
        }
    }

    true
}

fn calculate_middle_page_total(file_path: Utf8PathBuf) -> i32 {
    info!("Calculating Valid Updates middle page number total");
    let (rules, updates) = parse_file(file_path);

    let mut count = 0;

    for update in updates {
        if validate_update_order(&update, &rules) {
            info!("Update is Valid, update '{:?}'", update);
            let middle_page_num = update[(update.len() - 1) / 2];
            info!("Adding Middle page number to count {}", middle_page_num);
            count += middle_page_num
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    fn return_rules() -> Vec<Rule> {
        vec![
            vec![47, 53],
            vec![97, 13],
            vec![97, 61],
            vec![97, 47],
            vec![75, 29],
            vec![61, 13],
            vec![75, 53],
            vec![29, 13],
            vec![97, 29],
            vec![53, 29],
            vec![61, 53],
            vec![97, 53],
            vec![61, 29],
            vec![47, 13],
            vec![75, 47],
            vec![97, 75],
            vec![47, 61],
            vec![75, 61],
            vec![47, 29],
            vec![75, 13],
            vec![53, 13],
        ]
    }
    #[test]
    fn test_sample_input_parsing() {
        test_init();

        assert_eq!(
            (
                return_rules(),
                vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ]
            ),
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day5_sample.txt"))
        );
    }

    #[test]
    fn test_validate_order_sample_update_1_valid() {
        test_init();

        assert!(validate_update_order(
            &vec![75, 47, 61, 53, 29],
            &return_rules()
        ))
    }

    #[test]
    fn test_validate_order_sample_update_2_valid() {
        test_init();

        assert!(validate_update_order(
            &vec![97, 61, 53, 29, 13],
            &return_rules()
        ))
    }

    #[test]
    fn test_validate_order_sample_update_3_valid() {
        test_init();

        assert!(validate_update_order(&vec![75, 29, 13], &return_rules()))
    }

    #[test]
    fn test_validate_order_sample_update_4_invalid() {
        test_init();

        assert_eq!(
            false,
            validate_update_order(&vec![75, 97, 47, 61, 53], &return_rules())
        )
    }

    #[test]
    fn test_validate_order_sample_update_5_invalid() {
        test_init();

        assert_eq!(
            false,
            validate_update_order(&vec![61, 13, 29], &return_rules())
        )
    }

    #[test]
    fn test_validate_order_sample_update_6_invalid() {
        test_init();

        assert_eq!(
            false,
            validate_update_order(&vec![97, 13, 75, 29, 47], &return_rules())
        )
    }

    #[test]
    fn test_calculate_middle_pages_total_from_sample() {
        test_init();

        assert_eq!(
            143,
            calculate_middle_page_total(Utf8PathBuf::from("./src/puzzle_inputs/day5_sample.txt"))
        )
    }
}
