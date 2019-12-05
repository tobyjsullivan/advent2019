use std::env;
use std::io;
use std::io::Write;
use std::fs;

type Base = i64;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply font file.");
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file.");


    let result = interpret(input.as_ref());
    println!("{:?}", result);
}

fn read_user_input() -> Base {
    print!("INPUT: ");
    io::stdout().flush().expect("Error during flush.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Failed to parse input.")
}

fn interpret(input: &str) -> Vec<Base> {
    // Split the input on commas
    let codes: Vec<Base> = input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse().expect("Not a number."))
        .collect();

    // Execute computations
    compute(&codes, 0)
}

fn compute(input: &Vec<Base>, cursor: usize) -> Vec<Base> {
    let cmd = parse_instruction(input[cursor]).expect("Invalid command.");
    let mut result = input.to_vec();

    match cmd {
        Command::Add { p_modes } => {
            op_add(&mut result, p_modes, cursor);

            compute(&result, cursor + 4)
        }
        Command::Multiply { p_modes } => {
            op_mult(&mut result, p_modes, cursor);

            compute(&result, cursor + 4)
        }
        Command::Input => {
            op_input(&mut result, cursor);

            compute(&result, cursor + 2)
        }
        Command::Output { p_modes } => {
            op_output(&result, p_modes, cursor);

            compute(&result, cursor + 2)
        }
        Command::Halt => result,
    }
}

fn get(input: &Vec<Base>, p_mode: PMode, idx: usize) -> Base {
    match p_mode {
        PMode::Position => {
            let pos = input[idx] as usize;
            input[pos]
        }
        PMode::Immediate => {
            input[idx]
        }
    }
}

fn op_add(input: &mut Vec<Base>, p_modes: [PMode; 2], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    let b = get(input, p_modes[1], pc + 2);
    let out = input[pc + 3] as usize;
    input[out] = a + b;
}

fn op_mult(input: &mut Vec<Base>, p_modes: [PMode; 2], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    let b = get(input, p_modes[1], pc + 2);
    let out = input[pc + 3] as usize;
    input[out] = a * b;
}

fn op_input(input: &mut Vec<Base>, pc: usize) {
    let out = input[pc + 1] as usize;
    input[out] = read_user_input();
}

fn op_output(input: &Vec<Base>, p_modes: [PMode; 1], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    println!("OUTPUT: {}", a);
}

fn parse_instruction(input: Base) -> Result<Command, String> {
    let opcode = input % 100;
    let mut rem = input / 100;
    let mut p_modes = Vec::<PMode>::new();
    while rem > 0 {
        let p_mode = match rem % 10 {
            0 => Ok(PMode::Position),
            1 => Ok(PMode::Immediate),
            _ => Err("Unknown PMODE"),
        }?;
        p_modes.push(p_mode);
        rem /= 10;
    }

    match opcode {
        1 => Ok(Command::Add {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        2 => Ok(Command::Multiply{
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        3 => Ok(Command::Input),
        4 => Ok(Command::Output {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
            ]
        }),
        99 => Ok(Command::Halt),
        _ => Err("Unknown OPCODE".to_owned()),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Command {
    Add {
        p_modes: [PMode; 2],
    },
    Multiply {
        p_modes: [PMode; 2],
    },
    Input,
    Output {
        p_modes: [PMode; 1],
    },
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PMode {
    Position,
    Immediate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd() {
        let expected = Ok(Command::Add {
            p_modes: [
            PMode::Position,
            PMode::Position,
        ]});
        assert_eq!(parse_instruction(1), expected);
    }

    #[test]
    fn test_parse_cmd_modes() {
        let expected = Ok(Command::Multiply {
            p_modes: [
            PMode::Position,
            PMode::Immediate,
        ]});
        assert_eq!(parse_instruction(1002), expected);
    }

    #[test]
    fn test_add() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        op_add(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[5, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_add_neg() {
        let mut input = vec![1, 5, 6, 0, 99, 2, -3];
        op_add(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[-1, 5, 6, 0, 99, 2, -3]);
    }

    #[test]
    fn test_add_imm() {
        let mut input = vec![1, 5, 6, 3, 99, 2, 3];
        op_add(&mut input, [PMode::Immediate, PMode::Position], 0);

        assert_eq!(input, &[1, 5, 6, 8, 99, 2, 3]);
    }

    #[test]
    fn test_mult() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        op_mult(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[6, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_interpret_case1() {
        let result = interpret("1,0,0,0,99");
        assert_eq!(result, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_interpret_case2() {
        let result = interpret("2,3,0,3,99");
        assert_eq!(result, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_interpret_case3() {
        let result = interpret("2,4,4,5,99,0");
        assert_eq!(result, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_interpret_case4() {
        let result = interpret("1,1,1,4,99,5,6,0,99");
        assert_eq!(result, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
