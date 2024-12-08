use camino::Utf8PathBuf;

use crate::read_file;

fn parse_file(file_path: Utf8PathBuf) -> Vec<(i32, Vec<i32>)> {
    let mut equations = vec![];

    let content = read_file(file_path);
    let equation_strings = content.split("\n");
    for equation in equation_strings {
        let parts: Vec<&str> = equation.split(": ").collect();
        let total = parts[0].parse::<i32>().expect("To be a valid number");
        let values: Vec<i32> = parts[1]
            .split(" ")
            .map(|value| value.parse::<i32>().expect("To be a valid number"))
            .collect();
        equations.push((total, values));
    }

    equations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_init;

    #[test]
    fn test_file_input() {
        test_init();

        assert_eq!(
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ],
            parse_file(Utf8PathBuf::from("./src/puzzle_inputs/day7_sample.txt"))
        )
    }
}
