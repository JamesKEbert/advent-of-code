use camino::Utf8PathBuf;
use clap::Subcommand;
use regex::Regex;

use crate::read_file;

#[derive(Subcommand, Debug)]
pub enum Day3Commands {
    /// Multiplies Valid Memory
    MultiplyValidMemory {
        /// Input File Path
        #[arg(short, long)]
        path: Utf8PathBuf,
    },
}

pub fn day3_cli_command_processing(command: &Day3Commands) {
    match command {
        Day3Commands::MultiplyValidMemory { path } => {
            info!("Command received to multiply valid memory");
            println!(
                "Total from multiplications: {}",
                multiply_valid_memory(path.clone())
            );
        }
    }
}

fn match_valid_memory(memory: String) -> Vec<String> {
    info!("Identifying valid memory");
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").expect("To be a valid regex");
    let valid_memory = regex
        .find_iter(&memory)
        .map(|m| m.as_str().to_owned())
        .collect();
    info!("Calculated Valid Memory");
    debug!("Valid Memory: {:?}", valid_memory);
    valid_memory
}

fn multiply_valid_memory(file_path: Utf8PathBuf) -> i32 {
    info!("Multiplying Valid Memory...");
    let content = read_file(file_path);
    debug!("Memory: {:?}", content);

    let valid_memory = match_valid_memory(content);
    let mut count = 0;

    for memory in valid_memory {
        let trimmed_memory =
            memory.trim_matches(|c| c == 'm' || c == 'u' || c == 'l' || c == '(' || c == ')');
        debug!("Trimmed Memory: {}", trimmed_memory);
        let split_memory: Vec<i32> = trimmed_memory
            .split(",")
            .map(|level| level.parse::<i32>().expect("To be a valid number"))
            .collect();

        count += split_memory[0] * split_memory[1];
    }

    info!("Calculated Multiply Total: {}", count);
    count
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    #[test]
    fn test_corrupted_memory() {
        test_init();
        assert_eq!(
            vec![] as Vec<String>,
            match_valid_memory(String::from("mul(4*"))
        );
        assert_eq!(
            vec![] as Vec<String>,
            match_valid_memory(String::from("mul(6,9!"))
        );
        assert_eq!(
            vec![] as Vec<String>,
            match_valid_memory(String::from("?(12,34)"))
        );
        assert_eq!(
            vec![] as Vec<String>,
            match_valid_memory(String::from("mul ( 2 , 4 )"))
        );
    }

    #[test]
    fn test_sample_match_memory() {
        test_init();

        assert_eq!(
            vec![
                String::from("mul(2,4)"),
                String::from("mul(5,5)"),
                String::from("mul(11,8)"),
                String::from("mul(8,5)")
            ] as Vec<String>,
            match_valid_memory(String::from(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ))
        );
    }

    #[test]
    fn test_sample_multiply_valid_memory() {
        test_init();
        assert_eq!(
            161,
            multiply_valid_memory(Utf8PathBuf::from("./src/puzzle_inputs/day3_sample.txt"))
        );
    }
}
