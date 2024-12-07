use std::collections::HashMap;

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
        /// Whether to calculate from the valid or invalid updates
        #[arg(long, default_value_t = false)]
        invalid_updates: bool,
    },
}

pub fn day5_cli_command_processing(command: &Day5Commands) {
    match command {
        Day5Commands::CalculateMiddlePages {
            path,
            invalid_updates,
        } => {
            info!("Command received to calculate middle pages total");
            println!(
                "Total Number of Middle Pages from updates: {}",
                calculate_middle_page_total(path.clone(), invalid_updates.to_owned())
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

fn calculate_middle_page_total(file_path: Utf8PathBuf, invalid_updates: bool) -> i32 {
    info!("Calculating Valid Updates middle page number total");
    let (rules, updates) = parse_file(file_path);

    let mut count = 0;

    for update in updates {
        if validate_update_order(&update, &rules) {
            if !invalid_updates {
                info!("Update is Valid, update '{:?}'", update);
                let middle_page_num = update[(update.len() - 1) / 2];
                info!("Adding Middle page number to count {}", middle_page_num);
                count += middle_page_num;
            }
        } else {
            if invalid_updates {
                info!("Update is not valid, reordering update according to rules");
                let ordered_update = order_incorrect_update(&update, &rules);
                debug!(
                    "Valid order: {}",
                    validate_update_order(&ordered_update, &rules)
                );
                let middle_page_num = ordered_update[(ordered_update.len() - 1) / 2];
                info!("Adding Middle page number to count {}", middle_page_num);
                count += middle_page_num;
            }
        }
    }
    count
}
type RuleList = Vec<i32>;
fn compute_rule_order(rules: &mut Vec<Rule>, relevant_rules: &Vec<i32>) -> RuleList {
    info!("Computing Correct Rule Order");
    debug!("Rules: {:?}, relevant_rules: {:?}", rules, relevant_rules);

    rules.retain(|rule_pair| {
        if relevant_rules.contains(&rule_pair[0]) && relevant_rules.contains(&rule_pair[1]) {
            return true;
        } else {
            return false;
        }
    });
    debug!("Rules: {:?}, relevant_rules: {:?}", rules, relevant_rules);

    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule_pair in rules.clone() {
        debug!("Rule '{}'", rule_pair[0]);
        let mut rule_option = rule_map.get(&rule_pair[0]);
        let empty_rule_vec: Vec<i32> = vec![];
        let mut rule = rule_option.get_or_insert(&empty_rule_vec).to_owned();
        debug!("Rule_Option {:?}, rule {:?}", rule_option, rule);
        if !rule.contains(&rule_pair[1]) {
            rule.push(rule_pair[1]);
        }
        rule_map.insert(rule_pair[0], rule);
    }

    // Find the singular rule that is not left
    for rule in rules {
        if !rule_map.contains_key(&rule[1]) {
            rule_map.insert(rule[1], vec![]);
            break;
        }
    }

    // fn something() {
    //     let mut map = HashMap::new();
    //     map.insert(47, vec![53, 13, 61, 29]);
    //     map.insert(97, vec![13, 61, 47, 29, 53, 75]);
    //     map.insert(75, vec![29, 53, 47, 61, 13]);
    //     map.insert(61, vec![13, 53, 29]);
    //     map.insert(29, vec![13]);
    //     map.insert(53, vec![29, 13]);
    //     map
    // }
    // 97, 75, 47,
    // 97, 75, 47, 61, 53, 29, 13

    // 57, 31, 26, 42, 23, 64
    // 64, 57, 31

    // 57 | 31
    // 64 | 57
    // 31 | 64
    // 57 31 64
    info!("rule_map: '{:?}', length {}", rule_map, rule_map.len());
    let mut sorted_rules = vec![];

    for (rule, rule_pairs) in rule_map {
        debug!("Rule '{}' pairs: {:?}", rule, rule_pairs);
        if sorted_rules.len() == 0 {
            sorted_rules.insert(0, rule);
            info!("Sorted rules: '{:?}'", sorted_rules);
            continue;
        }

        'sorted_rules_loop: for i in 0..sorted_rules.len() {
            if rule_pairs.contains(&sorted_rules[i]) {
                debug!(
                    "Rule Pairs contains current sorted rule '{}', inserting '{}'",
                    sorted_rules[i], rule
                );
                sorted_rules.insert(i, rule);
                break 'sorted_rules_loop;
            } else {
                debug!(
                    "Rule Pairs do not contain current sorted rule '{}'",
                    sorted_rules[i]
                );
                if i + 1 == sorted_rules.len() {
                    debug!(
                        "Reached end of sorted rules, adding rule '{}' to end of sorted rules",
                        rule
                    );
                    sorted_rules.push(rule);
                }
            }
        }
        info!("Sorted rules: '{:?}'", sorted_rules);
    }

    info!("Sorted rules: '{:?}'", sorted_rules);

    sorted_rules
}

fn order_incorrect_update(update: &Update, rules: &Vec<Rule>) -> Update {
    info!("Reordering Incorrectly ordered update");
    debug!("Update: '{:?}', rules: '{:?}'", update, rules);

    let mut ordered_update: Update = vec![];
    let ordered_rules = compute_rule_order(&mut rules.clone(), update);
    for rule in ordered_rules {
        if update.contains(&rule) {
            ordered_update.push(rule.to_owned());
        }
    }

    info!("Ordered Update: '{:?}'", ordered_update);

    ordered_update
}

// fn compute_rule_order(rules: &Vec<Rule>) -> RuleList {
//   info!("Computing Correct Rule Order");
//   debug!("Rules: {:?}", &rules);

//   let mut computing_rules = rules.clone();

//   let mut sorted_rules = vec![];
//   let mut current_rule = computing_rules[0][0];
//   let mut iterator = 0;
//   'compute_loop: while !(computing_rules.is_empty()) {
//       if iterator >= 100000 {
//           panic!("Entered infinite loop while computing correct rule order");
//       }
//       debug!("Current Rule '{}', iterator '{}'", current_rule, iterator);
//       for rule in &computing_rules {
//           trace!(
//               "Current Rule '{}', rule to compare {:?}",
//               current_rule,
//               rule
//           );
//           if current_rule == rule[0] {
//               current_rule = rule[1];
//               iterator += 1;
//               continue 'compute_loop;
//           }
//       }
//       // We get here if we reached the end of the computing_rules list
//       debug!("Adding rule to rules list '{}'", current_rule);
//       sorted_rules.insert(0, current_rule);

//       debug!(
//           "Removing current_rule from all righthand side rules, computing_rules: {:?}",
//           computing_rules
//       );

//       // Last rule, add final lefthand rule
//       if computing_rules.len() <= 1 {
//           sorted_rules.insert(0, computing_rules[0][0]);
//           break;
//       }

//       computing_rules.retain(|rule| {
//           if rule[1] == current_rule {
//               return false;
//           } else {
//               return true;
//           }
//       });
//       debug!("Computing Rules: {:?}", computing_rules);
//       current_rule = computing_rules[0][0];
//       iterator += 1;
//   }

//   info!("Sorted rules: '{:?}'", sorted_rules);

//   sorted_rules
// }

// This also works (but didn't if you don't filter for the relevant rules first)
// fn compute_rule_order(rules: &mut Vec<Rule>, relevant_rules: &Vec<i32>) -> RuleList {
//     info!("Computing Correct Rule Order");

//     debug!("Rules: {:?}, relevant_rules: {:?}", rules, relevant_rules);

//     rules.retain(|rule_pair| {
//         if relevant_rules.contains(&rule_pair[0]) && relevant_rules.contains(&rule_pair[1]) {
//             return true;
//         } else {
//             return false;
//         }
//     });
//     debug!("Rules: {:?}, relevant_rules: {:?}", rules, relevant_rules);

//     let mut rule_counts: HashMap<i32, usize> = HashMap::new();
//     for rule in rules.clone() {
//         debug!("Rule '{}'", rule[0]);
//         let mut count_option = rule_counts.get(&rule[0]);
//         let mut count = count_option.get_or_insert(&0).to_owned();
//         debug!("Count_Option {:?}, count {}", count_option, count);
//         count += 1;
//         rule_counts.insert(rule[0], count);
//     }

//     // Find the singular rule that is not left
//     for rule in rules {
//         if !rule_counts.contains_key(&rule[1]) {
//             rule_counts.insert(rule[1], 0);
//             break;
//         }
//     }

//     info!(
//         "rule counts: '{:?}', length {}",
//         rule_counts,
//         rule_counts.len()
//     );
//     let mut sorted_rules = vec![0; rule_counts.len()];
//     for (rule, count) in rule_counts {
//         debug!("Rule '{}', count '{}'", rule, count);
//         sorted_rules[count] = rule;
//     }
//     sorted_rules.reverse();
//     info!("Sorted rules: '{:?}'", sorted_rules);

//     sorted_rules
// }

// pt2 attempt 1:
// fn order_incorrect_update(update: &Update, rules: &Vec<Rule>) -> Update {
//     info!("Reordering Incorrectly ordered update");
//     debug!("Update: '{:?}', rules: '{:?}'", update, rules);

//     let mut ordered_update: Update = update.clone();
//     let mut iterations = 0;
//     while !validate_update_order(&ordered_update, rules) && iterations < 100 {
//         debug!(
//             "Invalid Update Order, reapplying rules, iteration {}",
//             iterations
//         );
//         for rule in rules {
//             if update.contains(&rule[0]) && update.contains(&rule[1]) {
//                 // debug!(
//                 //     "Update contains Page Numbers corresponding to Rule '{},{}'",
//                 //     rule[0], rule[1]
//                 // );

//                 // There's probably better ways to do this
//                 for (index, page_num) in ordered_update.clone().iter().enumerate() {
//                     if page_num == &rule[0] {
//                         // debug!("This rule is properly ordered");
//                         continue;
//                     }
//                     if page_num == &rule[1] && index + 1 < ordered_update.len() {
//                         debug!("Incrementing page_num, index {}", index);
//                         ordered_update.swap(index, index + 1);
//                     }
//                     debug!("Ordered Update: {:?}", ordered_update);
//                 }
//             }
//         }
//         iterations += 1;
//     }

//     ordered_update
// }

// pt2 attempt 2:
// fn order_incorrect_update(update: &Update, rules: &Vec<Rule>) -> Update {
//     info!("Reordering Incorrectly ordered update");
//     debug!("Update: '{:?}', rules: '{:?}'", update, rules);

//     let mut ordered_update: Update = update.clone();

//     let mut map = HashMap::new();
//     map.insert(47, vec![53, 13, 61, 29]);
//     map.insert(97, vec![13, 61, 47, 29, 53, 75]);
//     map.insert(75, vec![29, 53, 47, 61, 13]);
//     map.insert(61, vec![13, 53, 29]);
//     map.insert(29, vec![13]);
//     map.insert(53, vec![29, 13]);

//     for (left_rule, right_rules) in map.iter() {
//         debug!(
//             "For Rule '{}' with corresponding right-hand pairs '{:?}",
//             left_rule, right_rules
//         );
//         if ordered_update.contains(left_rule) {
//             // This loop is just to find the left_rule page number index
//             let mut left_rule_page_num_index: usize = 0;
//             for (index, page_num) in ordered_update.iter().enumerate() {
//                 if page_num == left_rule {
//                     left_rule_page_num_index = index;
//                     continue;
//                 }
//             }

//             if ordered_update[0..left_rule_page_num_index].contains
//         }
//     }

//     ordered_update
// }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_init;

    // fn return_sorted_rules() -> HashMap<i32, Vec<i32>> {
    //     let mut map = HashMap::new();
    //     map.insert(47, vec![53, 13, 61, 29]);
    //     map.insert(97, vec![13, 61, 47, 29, 53, 75]);
    //     map.insert(75, vec![29, 53, 47, 61, 13]);
    //     map.insert(61, vec![13, 53, 29]);
    //     map.insert(29, vec![13]);
    //     map.insert(53, vec![29, 13]);
    //     map
    // }

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
            calculate_middle_page_total(
                Utf8PathBuf::from("./src/puzzle_inputs/day5_sample.txt"),
                false
            )
        )
    }

    #[test]
    fn test_reordering_invalid_update_1() {
        test_init();

        assert_eq!(
            vec![97, 75, 47, 61, 53],
            order_incorrect_update(&vec![75, 97, 47, 61, 53], &return_rules())
        )
    }

    #[test]
    fn test_reordering_invalid_update_2() {
        test_init();

        assert_eq!(
            vec![61, 29, 13],
            order_incorrect_update(&vec![61, 13, 29], &return_rules())
        )
    }

    #[test]
    fn test_reordering_invalid_update_3() {
        test_init();

        assert_eq!(
            vec![97, 75, 47, 29, 13,],
            order_incorrect_update(&vec![97, 13, 75, 29, 47], &return_rules())
        )
    }

    #[test]
    fn test_computing_rule_order_sample() {
        test_init();

        assert_eq!(
            vec![97, 75, 47, 61, 53, 29, 13],
            compute_rule_order(&mut return_rules(), &mut vec![97, 75, 47, 61, 53, 29, 13])
        )
    }

    // Do you think I could make this function name longer? ;)
    #[test]
    fn test_calculate_middle_pages_total_from_sample_reordering_incorrect_updates() {
        test_init();

        assert_eq!(
            123,
            calculate_middle_page_total(
                Utf8PathBuf::from("./src/puzzle_inputs/day5_sample.txt"),
                true
            )
        )
    }
}
