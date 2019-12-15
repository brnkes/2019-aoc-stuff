use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod interpreter;
mod world;
mod arcade;

#[cfg(target_arch = "wasm32")]
mod wasm;

use interpreter::{Interpreter, InterpreterProcessResult};
use world::{WorldState, Color, Turn};
use arcade::Arcade;
use world::Canvas;

const NUM_AMPS: i64 = 1;

#[wasm_bindgen]
extern {
    fn await_input() -> i64;
}

pub struct Game {
    amp: Interpreter,
    arcade: Arcade,
    watchdog: i32,
    pub mem_input: Rc<RefCell<VecDeque<i64>>>,
    pub mem_output: Rc<RefCell<VecDeque<i64>>>,
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub struct Game {
    amp: Interpreter,
    arcade: Arcade,
    watchdog: i32,
    mem_input: Rc<RefCell<VecDeque<i64>>>,
    mem_output: Rc<RefCell<VecDeque<i64>>>,
}

#[wasm_bindgen]
impl Game {
    pub fn initialize(input: String) -> Game {
        Game::generate_and_run_amps(&input)
    }

    fn generate_and_run_amps(input: &str) -> Game {
        let codes: Vec<i64> = input
            .split(",")
            .map(|code_txt| code_txt.parse::<i64>().unwrap())
            .collect();

        let codes_copy = codes.clone();

        let mem_input = Rc::new(RefCell::new(VecDeque::new()));

        {
            let mut q = mem_input.as_ref().borrow_mut();
        }

        let mem_output = Rc::new(RefCell::new(VecDeque::new()));

        let mut amp = Interpreter::new(
            0,
            0,
            codes_copy,
            mem_input.clone(),
            mem_output.clone(),
        );

        // run stuff

        let mut arcade = Arcade::new();
        let mut skipped_inputs_so_far = 0;
        let mut watchdog = 50000;

        Game {
            amp,
            arcade,
            watchdog,
            mem_input,
            mem_output
        }
    }

    pub fn get_arcade_size(&self) -> Canvas {
        self.arcade.get_canvas_size()
    }

    pub fn pass_input(&mut self, input: i64) {
        let mut q = self.mem_input.borrow_mut();
        match q.front_mut() {
            None => { q.push_back(input) },
            Some(front) => {
                *front = input
            },
        }
    }

    pub fn loop_once(&mut self) -> InterpreterProcessResult {
        let Game {
            amp,
            arcade,
            watchdog,
            mem_input,
            mem_output
        } = self;

        let pass_to_next = amp.process();

        match pass_to_next {
            InterpreterProcessResult::ThreeOutputs => {
                assert_eq!(mem_output.borrow().len(), 3, "Should've outputted 3 values.");

                {
                    let mut output_q = mem_output.borrow_mut();

                    let x = output_q.pop_front().unwrap();
                    let y = output_q.pop_front().unwrap();
                    let tile_id = output_q.pop_front().unwrap();

                    arcade.draw_stuff(x,y,tile_id as u64);
                }

                *watchdog -= 1;

                if (*watchdog) < 0 {
                    panic!("Reached max loop limit");
                }
            },
            _ => {
                //nothing. js will handle in/end
            },
        }

        pass_to_next
    }

}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
}


