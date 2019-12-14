//#![feature(trace_macros)]

//trace_macros!(true);

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
#[derive(Debug)]
enum Op {
    Sum,
    Mul,
    End,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
}

impl Op {
    fn parse_opcode(op_code: i64) -> Result<Op, String> {
        match op_code {
            1 => Ok(Op::Sum),
            2 => Ok(Op::Mul),
            3 => Ok(Op::Input),
            4 => Ok(Op::Output),
            5 => Ok(Op::JumpIfTrue),
            6 => Ok(Op::JumpIfFalse),
            7 => Ok(Op::LessThan),
            8 => Ok(Op::Equals),
            9 => Ok(Op::AdjustRelativeBase),
            99 => Ok(Op::End),
            _ => Err(format!("Wrong opcode : {}", op_code))
        }
    }

    fn param_count(&self) -> usize {
        match self {
            Op::Sum => 3,
            Op::Mul => 3,
            Op::End => 0,
            Op::Input => 1,
            Op::Output => 1,
            Op::JumpIfTrue => 2,
            Op::JumpIfFalse => 2,
            Op::LessThan => 3,
            Op::Equals => 3,
            Op::AdjustRelativeBase => 1
        }
    }

    fn get_binary_i64_op(&self) -> Box<dyn Fn(i64, i64) -> i64> {
        match self {
            Op::Sum => {
                (Box::new(|arg1, arg2| arg1 + arg2))
            }
            Op::Mul => {
                (Box::new(|arg1, arg2| arg1 * arg2))
            }
            Op::LessThan => {
                (Box::new(|arg1, arg2| if arg1 < arg2 { 1 } else { 0 }))
            }
            Op::Equals => {
                (Box::new(|arg1, arg2| if arg1 == arg2 { 1 } else { 0 }))
            }
            _ => panic!("Invalid arithmetic op : {:?}", self),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

macro_rules! check_and_resize {
    ( $program:expr, $targetted_index:expr ) => {
//        println!("{} <=> {}", $targetted_index, $program.len());

        #[allow(unused_comparisons)]
        let is_negative_idx = $targetted_index < 0;
        assert!(!is_negative_idx, "Cannot access negative index.");

        let idx_comp = $targetted_index as usize;

        if idx_comp as usize >= $program.len() {
            $program.resize(idx_comp + 1, 0);
        }
    };
}


impl Mode {
    fn parse_modecode(mode_code: &i64) -> Result<Mode, String> {
        match *mode_code {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(format!("Wrong modecode : {}", mode_code))
        }
    }

    fn read_with(&self, program: &mut Vec<i64>, rel_ptr: i64, idx: usize) -> i64 {
        match self {
            Mode::Immediate => {
                check_and_resize!(program,idx);

                program[idx]
            },
            Mode::Position => {
                check_and_resize!(program,idx);
                check_and_resize!(program,program[idx]);

                program[program[idx] as usize]
            },
            Mode::Relative => {
                check_and_resize!(program,idx);
                let idx_final = (rel_ptr + program[idx]) as usize;
                check_and_resize!(program,idx_final);

                program[idx_final as usize]
            }
        }
    }

    fn write_with(&self, program: &mut Vec<i64>, rel_ptr: i64, idx: usize, value: i64) {
        match self {
            Mode::Immediate => panic!("Cannot write in immediate mode."),
            Mode::Position => {
                check_and_resize!(program,idx);
                let ptr = program[idx];
                check_and_resize!(program,ptr);

                program[ptr as usize] = value
            },
            Mode::Relative => {
                check_and_resize!(program,idx);
                let idx_final = (rel_ptr + program[idx]) as usize;
                check_and_resize!(program,idx_final);

                program[idx_final as usize] = value
            }
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct OpAndModes {
    op_code: Op,
    modes: Vec<Mode>,
}

trait MemShorthand {
    fn read_mem(
        &self, program: &mut Vec<i64>, idx: usize, rel_base_ptr: i64, exec_ptr: usize
    ) -> i64;

    fn write_mem(
        &self, program: &mut Vec<i64>, idx: usize, rel_base_ptr: i64, exec_ptr: usize,
        value_fun: &dyn Fn() -> i64,
    );
}

impl MemShorthand for Vec<Mode> {
    fn read_mem(&self, program: &mut Vec<i64>, idx: usize, rel_base_ptr: i64, exec_ptr: usize) -> i64 {
        self.get(idx).unwrap_or(&Mode::Position).read_with(
            program, rel_base_ptr, exec_ptr + idx + 1,
        )
    }

    fn write_mem(
        &self, program: &mut Vec<i64>, idx: usize, rel_base_ptr: i64, exec_ptr: usize,
        value_fun: &dyn Fn() -> i64,
    ) {
        self.get(idx).unwrap_or(&Mode::Position).write_with(
            program,
            rel_base_ptr,
            exec_ptr + idx + 1,
            value_fun(),
        );
    }
}

fn eval(
    program: &mut Vec<i64>,
    handle_output: &mut dyn FnMut(i64),
    handle_input: &mut dyn FnMut() -> Option<i64>,
    exec_ptr: &mut usize,
    rel_base_ptr: &mut i64,
    opcode_and_modecodes: OpAndModes,
) {
    let op_code = opcode_and_modecodes.op_code;
    let param_count = op_code.param_count();
    let modes = opcode_and_modecodes.modes;

    match op_code {
        Op::Sum | Op::Mul | Op::Equals | Op::LessThan => {
            let arg1 = modes.read_mem(program, 0, *rel_base_ptr, *exec_ptr);
            let arg2 = modes.read_mem(program, 1, *rel_base_ptr, *exec_ptr);

            modes.write_mem(
                program, 2, *rel_base_ptr, *exec_ptr,
                &|| {
                    op_code.get_binary_i64_op()(arg1, arg2)
                }
            );
        }
        // todo : bools
        Op::Input => {
            let incoming_value_opt = handle_input();

            match incoming_value_opt {
                None => {
                    // Will retry... don't change exec ptr.
                    return;
                }
                Some(incoming_value) => {
                    modes.write_mem(
                        program, 0, *rel_base_ptr, *exec_ptr,
                        &|| {
                            incoming_value
                        }
                    );
                }
            }
        }
        Op::Output => {
            let to_output = modes.read_mem(program, 0, *rel_base_ptr, *exec_ptr);
            handle_output(to_output);
        }
        Op::JumpIfTrue => {
            if modes.read_mem(program, 0, *rel_base_ptr, *exec_ptr) != 0 {
                *exec_ptr = modes.read_mem(program, 1, *rel_base_ptr, *exec_ptr) as usize;
                return;
            }
        }
        Op::JumpIfFalse => {
            if modes.read_mem(program, 0, *rel_base_ptr, *exec_ptr) == 0 {
                *exec_ptr = modes.read_mem(program, 1, *rel_base_ptr, *exec_ptr) as usize;
                return;
            }
        }
        Op::End => panic!("Should've been handled outside the eval block."),
        Op::AdjustRelativeBase => {
            *rel_base_ptr += modes.read_mem(program, 0, *rel_base_ptr, *exec_ptr);
        }
    }

    *exec_ptr += param_count + 1;
}

fn extract_op_and_modes(input: i64) -> OpAndModes {
    let op_and_mode = input;
    let op_code = op_and_mode % 100;
    let mut mode = op_and_mode / 100;

    let mut modes = Vec::new();
    while mode > 0 {
        modes.push(mode % 10);
        mode = mode / 10;
    }

    OpAndModes {
        op_code: Op::parse_opcode(op_code).unwrap(),
        modes: modes.iter().map(|code| Mode::parse_modecode(code).unwrap()).collect(),
    }
}

#[wasm_bindgen]
pub enum InterpreterProcessResult {
    ThreeOutputs = 0,
    WaitingOnInput = 1,
    Ended = 2
}

pub struct Interpreter {
    exec_ptr: usize,
    rel_base_ptr: i64,
    memory: Vec<i64>,
    input_queue: Rc<RefCell<VecDeque<i64>>>,
    output_queue: Rc<RefCell<VecDeque<i64>>>,
}

impl Interpreter {
    pub fn new(
        exec_ptr: usize,
        rel_base_ptr: i64,
        memory: Vec<i64>,
        input_queue: Rc<RefCell<VecDeque<i64>>>,
        output_queue: Rc<RefCell<VecDeque<i64>>>,
    ) -> Interpreter {
        Interpreter {
            exec_ptr,
            rel_base_ptr,
            memory,
            input_queue,
            output_queue,
        }
    }

    pub fn get_last_output(&self) -> i64 {
        self.output_queue.as_ref().borrow().back().unwrap().clone()
    }

    pub fn process(&mut self) -> InterpreterProcessResult {
        let output_semcount = Rc::new(Cell::new(0));
        let input_wait = Rc::new(Cell::new(false));

        let output_queue = self.output_queue.clone();
        let input_queue = self.input_queue.clone();

        let mut handle_output = |outgoing: i64| {
            output_queue.as_ref().borrow_mut().push_back(outgoing);
            let mut sc_r = output_semcount.as_ref();
            let mut sc = sc_r.borrow_mut();
            sc.set(sc.get() + 1);
        };

        let mut handle_input = || {
            let res = input_queue.as_ref().borrow_mut().pop_front();

            if let None = res {
                input_wait.as_ref().borrow_mut().set(true);
            }

            res
        };

        while self.exec_ptr < self.memory.len() {
            let opcode_and_modecodes = extract_op_and_modes(self.memory[self.exec_ptr]);

            if opcode_and_modecodes.op_code == Op::End {
                println!("Terminate ?");
                return InterpreterProcessResult::Ended;
            }

            eval(
                &mut self.memory,
                &mut handle_output,
                &mut handle_input,
                &mut self.exec_ptr, &mut self.rel_base_ptr, opcode_and_modecodes,
            );

            if output_semcount.as_ref().borrow().get() >= 3 {
                return InterpreterProcessResult::ThreeOutputs;
            }

            if  input_wait.as_ref().borrow().get() {
                return InterpreterProcessResult::WaitingOnInput;
            }
        }

        return InterpreterProcessResult::Ended;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_extraction() {
        let mut result = extract_op_and_modes(1002);
        assert_eq!(result, OpAndModes {
            op_code:Op::Mul,
            modes:vec![Mode::Position,Mode::Immediate]
        });

        result = extract_op_and_modes(109);
        assert_eq!(result, OpAndModes {
            op_code:Op::AdjustRelativeBase,
            modes:vec![Mode::Immediate]
        })
    }

//    #[test]
//    fn test_traverse() {
//        let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
//        let mut input_queue = VecDeque::new();
//        input_queue.push_front(1);
//        traverse(&mut program, &mut input_queue);
//
//        print!("{:?}", program);
//    }
}

