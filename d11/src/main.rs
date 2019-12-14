use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::interpreter::Interpreter;
use crate::world::{WorldState, Color, Turn};

mod interpreter;
mod world;

const NUM_AMPS: i64 = 1;

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let result = process(
        input
    );

    println!("{}", result);
}

fn process(input: String) -> usize {
    generate_and_run_amps(&input)
}

fn generate_and_run_amps(input: &str) -> usize {
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

//        println!("Interpreter created : {} ... {}",
//            amp_idx,
//            mem_input.as_ref().borrow().front().unwrap()
//        );

    let mut amp = Interpreter::new(
        0,
        0,
        codes_copy,
        mem_input.clone(),
        mem_output.clone(),
    );

    // run stuff

    let mut world = WorldState::fresh();

    let mut watchdog = 50000;
    loop {
        {
            let camera = world.camera();
            mem_input.borrow_mut().push_back(camera);
        }

        let pass_to_next = amp.process();

        // get result
        if !pass_to_next {
            break;
        }

        assert!(mem_output.borrow().len() >= 2, "Should've outputted 2 values.");

        {
            let mut output_q = mem_output.borrow_mut();

            let color = output_q.pop_front().unwrap();
            world.paint(Color::convert(color));

            let turn_dir = output_q.pop_front().unwrap();
            world.turn(Turn::convert(turn_dir));

            world.translate();
        }

        watchdog -= 1;

        if (watchdog) < 0 {
            panic!("Reached max loop limit");
        }
    }

    world.visualize_visited_coords();
    world.get_visited_coords_count()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
}


