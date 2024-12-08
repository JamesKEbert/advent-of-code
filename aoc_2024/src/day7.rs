use std::fmt::Display;

use camino::Utf8PathBuf;

use crate::read_file;

type Equation = (i32, Vec<i32>);
#[derive(Clone, Debug)]
enum Operator {
    Multiply,
    Add,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

fn parse_file(file_path: Utf8PathBuf) -> Vec<Equation> {
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

fn test_equation((total, values): &Equation, operators: &Vec<Operator>) -> bool {
    debug!(
        "Testing Equation '{:?}' with Operators {:?}",
        (total, values),
        operators
    );
    let mut calculated_total = values[0];
    let mut operator_index = 0;
    for value in values[1..values.len()].iter() {
        match operators[operator_index] {
            Operator::Add => {
                calculated_total = calculated_total + value;
            }
            Operator::Multiply => {
                calculated_total = calculated_total * value;
            }
        }
        operator_index += 1;
    }

    debug!(
        "Calculated Total: {}, Target Total: {}",
        calculated_total, total
    );
    &calculated_total == total
}

fn recursive_operator_test(
    equation: &Equation,
    adjusting_index: usize,
    mut operators: &mut Vec<Operator>,
) -> bool {
    if adjusting_index == operators.len() - 1 {
        operators[adjusting_index] = Operator::Add;
        if test_equation(equation, &operators) {
            return true;
        } else {
            operators[adjusting_index] = Operator::Multiply;
            return test_equation(equation, &operators);
        }
    } else {
        operators[adjusting_index] = Operator::Add;
        if recursive_operator_test(equation, adjusting_index + 1, operators) {
            return true;
        } else {
            operators[adjusting_index] = Operator::Multiply;
            return recursive_operator_test(equation, adjusting_index + 1, operators);
        }
    }
}

fn try_equation_operators(equation: &Equation) -> bool {
    let mut operators = vec![Operator::Add; equation.1.len() - 1];
    return recursive_operator_test(equation, 0, &mut operators);
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

    #[test]
    fn test_valid_function() {
        test_init();
        assert!(test_equation(
            &(3267, vec![81, 40, 27]),
            &vec![Operator::Add, Operator::Multiply]
        ))
    }

    #[test]
    fn test_equation_operators() {
        test_init();
        assert!(try_equation_operators(&(292, vec![11, 6, 16, 20])))
    }

    #[test]
    fn test_unsolveable_equation() {
        test_init();
        assert_eq!(false, try_equation_operators(&(161011, vec![16, 10, 13])))
    }
}
