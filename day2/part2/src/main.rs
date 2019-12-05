use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    'outer: for i in 0..100 {
        for j in 0..100 {
            let result = interpret(input.as_ref(), i, j);

            if result[0] == 19690720 {
                println!("Noun: {}; Verb: {}", i, j);
                println!("{:?}", result);
                break 'outer;
            }
        }
    }
}

fn interpret(input: &str, noun: usize, verb: usize) -> Vec<usize> {
    // Split the input on commas
    let mut codes: Vec<usize> = input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse().expect("Not a number."))
        .collect();

    live_swap(&mut codes, noun, verb);

    // Execute computations
    compute(&codes, 0)
}

fn live_swap(input: &mut Vec<usize>, noun: usize, verb: usize) {
    input[1] = noun;
    input[2] = verb;
}

fn compute(input: &Vec<usize>, cursor: usize) -> Vec<usize> {
    let cmd = parse_cmd(input[cursor]).expect("Invalid command.");
    let mut result = input.to_vec();

    match cmd {
        Command::Add => {
            add(&mut result, cursor);

            compute(&result, cursor + 4)
        }
        Command::Multiply => {
            mult(&mut result, cursor);

            compute(&result, cursor + 4)
        }
        Command::Halt => result,
    }
}

fn add(input: &mut Vec<usize>, pc: usize) {
    let pos_a = input[pc + 1];
    let pos_b = input[pc + 2];
    let out = input[pc + 3];
    let a = input[pos_a];
    let b = input[pos_b];
    input[out] = a + b;
}

fn mult(input: &mut Vec<usize>, pc: usize) {
    let pos_a = input[pc + 1];
    let pos_b = input[pc + 2];
    let out = input[pc + 3];
    let a = input[pos_a];
    let b = input[pos_b];
    input[out] = a * b;
}

fn parse_cmd(input: usize) -> Result<Command, ()> {
    match input {
        1 => Ok(Command::Add),
        2 => Ok(Command::Multiply),
        99 => Ok(Command::Halt),
        _ => Err(()),
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Add,
    Multiply,
    Halt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd() {
        assert_eq!(parse_cmd(1), Ok(Command::Add));
    }

    #[test]
    fn test_add() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        add(&mut input, 0);

        assert_eq!(input, &[5, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_mult() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        mult(&mut input, 0);

        assert_eq!(input, &[6, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_interpret_case1() {
        let result = interpret("1,0,0,0,99", 0, 0);
        assert_eq!(result, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_interpret_case2() {
        let result = interpret("2,3,0,3,99", 3, 0);
        assert_eq!(result, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_interpret_case3() {
        let result = interpret("2,4,4,5,99,0", 4, 4);
        assert_eq!(result, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_interpret_case4() {
        let result = interpret("1,1,1,4,99,5,6,0,99", 1, 1);
        assert_eq!(result, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
