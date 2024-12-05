use camino::Utf8PathBuf;

use crate::read_file;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    #[test]
    fn test_sample_input_parsing() {
        test_init();

        assert_eq!(
            (
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
                    vec![53, 13]
                ],
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
}
