use std::cell::RefCell;
use std::cmp::max;
use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

use super::interpreter::{Interpreter, InterpreterProcessResult};
use super::robot::{RobotState, Tile, Coords};
use super::arcade::Arcade;
use super::robot::Canvas;

#[cfg(target_arch = "wasm32")]
use wasm::util;

const NUM_AMPS: i64 = 1;

#[wasm_bindgen]
extern {
    fn await_input() -> i64;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Runner {
    interpreter: Interpreter,
    pub robot: RobotState,
    watchdog: i32,
    pub mem_input: Rc<RefCell<VecDeque<i64>>>,
    pub mem_output: Rc<RefCell<VecDeque<i64>>>,
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub struct Runner {
    interpreter: Interpreter,
    robot: RobotState,
    watchdog: i32,
    mem_input: Rc<RefCell<VecDeque<i64>>>,
    mem_output: Rc<RefCell<VecDeque<i64>>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Runner {
    pub fn initialize(input: String) -> Runner {
        #[cfg(target_arch = "wasm32")]
            util::set_panic_hook();

        Runner::generate_and_run_interpreter(&input)
    }

    fn generate_and_run_interpreter(input: &str) -> Runner {
        let codes: Vec<i64> = input
            .split(",")
            .map(|code_txt| code_txt.parse::<i64>().unwrap())
            .collect();

        let codes_copy = codes.clone();

        let mem_input = Rc::new(RefCell::new(VecDeque::new()));
        let mem_output = Rc::new(RefCell::new(VecDeque::new()));

        let mut interpreter = Interpreter::new(
            0,
            0,
            codes_copy,
            mem_input.clone(),
            mem_output.clone(),
        );

        let mut robot = RobotState::fresh();
        let mut skipped_inputs_so_far = 0;
        let mut watchdog = 50000;

        assert_eq!(mem_output.borrow().len(), 0, "Should have XXX values.");

        Runner {
            interpreter,
            robot,
            watchdog,
            mem_input,
            mem_output
        }
    }

    pub fn pass_input(&mut self, input: i64) {
        let mut q = self.mem_input.borrow_mut();
        q.push_back(input);
    }

    pub fn get_output(&mut self) -> Vec<i64> {
        const OUTPUT_QUEUE_QUERY_REPEAT_COUNT : u32 = 1;

        let mut q = self.mem_output.borrow_mut();

        let mut ret = Vec::new();
        for i in 0..OUTPUT_QUEUE_QUERY_REPEAT_COUNT {
            ret.push(q.pop_front().unwrap());
        }
        ret
    }

    pub fn execute_next_loop(&mut self) -> Option<u32> {
        let Runner {
            interpreter,
            robot,
            watchdog,
            mem_input,
            mem_output
        } = self;

        let next_move_r = robot.pick_next_walk_direction();

        match next_move_r {
            Err(_) => {
                println!("Ended search, measuring oxygen propagation...");
                return Some(robot.measure_fill_oxygen());
            }
            Ok(next_move) => {
                mem_input.borrow_mut().push_back(next_move as i64);

                let pass_to_next = interpreter.process();

                match pass_to_next {
                    InterpreterProcessResult::OneOutput => {
                        let robot_query_result = mem_output.borrow_mut().pop_front().unwrap();
                        let tile = Tile::convert(robot_query_result);
                        robot.interpret_robot_status(next_move, tile);

//                if let Tile::Oxygen = tile {
//                    println!("Oxygen found...");
//                    // return Some(robot.get_steps_to_origin())
//                }
                    },
                    InterpreterProcessResult::WaitingOnInput => {
                        panic!("Shouldn't wait for inputs in this loop mode.");
                    },
                    InterpreterProcessResult::Ended => {
                        panic!("Shouldn't end ; robot will decide the termination (?).");
                    }
                }

                return None
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
}


