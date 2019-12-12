use std::collections::VecDeque;

#[derive(PartialEq)]
#[derive(Debug)]
enum Op {
    Sum,
    Mul,
    End,
    Input,
    Output
}

impl Op {
    fn param_count(&self) -> usize {
        match self {
            Op::Sum => 3,
            Op::Mul => 3,
            Op::End => 0,
            Op::Input => 1,
            Op::Output => 1,
        }
    }

    fn get_arithmetic_op(&self) -> Option<Box<dyn Fn(i64,i64) -> i64>> {
        match self {
            Op::Sum => {
                Some(Box::new(|arg1, arg2| arg1 + arg2))
            }
            Op::Mul => {
                Some(Box::new(|arg1, arg2| arg1 * arg2))
            },
            _ => None,
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn read_with(&self, program: &[i64], idx: usize) -> i64 {
        match self {
            Mode::Immediate => program[idx],
            Mode::Position => program[program[idx] as usize]
        }
    }

    fn write_with(&self, program: &mut [i64], idx: usize, value: i64) {
        match self {
            Mode::Immediate => panic!("Cannot write in immediate mode."),
            Mode::Position => program[program[idx] as usize] = value
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct OpcodeAndModecodes {
    op_code:Op,
    modes:Vec<Mode>
}

fn parse_opcode(op_code:i64) -> Result<Op,String> {
    match op_code {
        1 => Ok(Op::Sum),
        2 => Ok(Op::Mul),
        3 => Ok(Op::Input),
        4 => Ok(Op::Output),
        99 => Ok(Op::End),
        _ => Err(String::from("Wrong opcode"))
    }
}

fn parse_modecode(mode_code:&i64) -> Result<Mode,String> {
    match *mode_code {
        0 => Ok(Mode::Position),
        1 => Ok(Mode::Immediate),
        _ => Err(String::from("Wrong mode code"))
    }
}

/*fn eval(op_code:i64, arg1:i64, arg2:i64) -> Result<i64,String> {
    let code = parse_opcode(op_code)?;
    Ok(get_op(code)(arg1, arg2))
}*/

fn eval(
    program: &mut [i64],
    handle_output: &dyn Fn(i64),
    handle_input: &mut dyn FnMut() -> i64,
    exec_ptr: &mut usize,
    opcode_and_modecodes: OpcodeAndModecodes,
) {
    let param_count = opcode_and_modecodes.op_code.param_count();
    let modes = opcode_and_modecodes.modes;

    match opcode_and_modecodes.op_code {
        Op::Sum | Op::Mul => {
            let arg1 = modes.get(0).unwrap_or(&Mode::Position).read_with(program, *exec_ptr + 1);
            let arg2 = modes.get(1).unwrap_or(&Mode::Position).read_with(program, *exec_ptr + 2);

            modes.get(2).unwrap_or(&Mode::Position).write_with(
                program,
                *exec_ptr + 3,
                opcode_and_modecodes.op_code.get_arithmetic_op().unwrap()(arg1, arg2)
            );
        },
        Op::Input => {
            let incoming_value = handle_input();
            modes.get(0).unwrap_or(&Mode::Position).write_with(
                program,
                *exec_ptr + 1,
                incoming_value
            );
        },
        Op::Output => {
            let to_output = modes.get(0).unwrap_or(&Mode::Position).read_with(program, *exec_ptr + 1);
            handle_output(to_output);
        },
        Op::End => panic!("Should've been handled outside the eval block."),
    }

    *exec_ptr += param_count + 1;
}

fn extract_op_and_modes(input: i64) -> OpcodeAndModecodes {
    let op_and_mode = input;
    let op_code = op_and_mode % 100;
    let mut mode = op_and_mode / 100;

    let mut modes = Vec::new();
    while mode > 0 {
        modes.push(mode % 10);
        mode = mode / 10;
    }

    OpcodeAndModecodes {
        op_code: parse_opcode(op_code).unwrap(),
        modes: modes.iter().map(|code| parse_modecode(code).unwrap()).collect()
    }
}

pub fn traverse(
    program:&mut [i64],
    input_queue:&mut VecDeque<i64>
) -> Result<(), String> {
    let handle_output = |outgoing:i64| {
        println!("Output >>> {}", outgoing);
    };

    let mut handle_input = || {
        input_queue.pop_front().unwrap()
    };

    let mut exec_ptr = 0;
    while exec_ptr < program.len() {
        let opcode_and_modecodes= extract_op_and_modes(program[exec_ptr]);

        if opcode_and_modecodes.op_code == Op::End {
            return Ok(());
        }

        eval(
            program, &handle_output, &mut handle_input,
            &mut exec_ptr, opcode_and_modecodes
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_extraction() {
        let result = extract_op_and_modes(1002);
        assert_eq!(result, OpcodeAndModecodes {
            op_code:Op::Mul,
            modes:vec![Mode::Position,Mode::Immediate]
        })
    }

    #[test]
    fn test_traverse() {
        let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
        let mut input_queue = VecDeque::new();
        input_queue.push_front(1);
        traverse(&mut program, &mut input_queue);

        print!("{:?}", program);
    }
}

