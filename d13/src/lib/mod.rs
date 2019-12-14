use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

mod interpreter;
mod world;
mod arcade;

use interpreter::Interpreter;
use world::{WorldState, Color, Turn};
use arcade::Arcade;

const NUM_AMPS: i64 = 1;

pub fn process(input: String) -> usize {
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

    let mut arcade = Arcade::new();

    let mut skipped_inputs_so_far = 0;

    let mut watchdog = 50000;
    loop {
        {
            if mem_input.borrow().len() < 1 {
                println!("Skipped : {}", skipped_inputs_so_far);

                skipped_inputs_so_far = 0;
                mem_input.borrow_mut().push_back(0);
            } else {
                skipped_inputs_so_far += 1;
            }
        }

        let pass_to_next = amp.process();

        // get result
        if !pass_to_next {
            break;
        }

        assert_eq!(mem_output.borrow().len(), 3, "Should've outputted 3 values.");

        {
            let mut output_q = mem_output.borrow_mut();

            let x = output_q.pop_front().unwrap();
            let y = output_q.pop_front().unwrap();
            let tile_id = output_q.pop_front().unwrap();

            arcade.draw_stuff(x,y,tile_id as u64);
        }

        watchdog -= 1;

        if (watchdog) < 0 {
            panic!("Reached max loop limit");
        }
    }

//    world.visualize_visited_coords();
    arcade.get_count_of(2) as usize
//    world.get_visited_coords_count()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
}


