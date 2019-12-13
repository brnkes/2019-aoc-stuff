use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::interpreter::Interpreter;

mod interpreter;

const NUM_AMPS: i64 = 1;

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let result = process(
        input
    );

    println!("Result : {}", result);
}

fn process(input: String) -> i64 {
    generate_and_run_amps(&input)
}

fn generate_and_run_amps(input: &str) -> i64 {
    let codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();

    let feedback_output = Rc::new(RefCell::new(VecDeque::new()));
    let mut last_output = None;
    let mut amps: Vec<Interpreter> = Vec::new();

    for amp_idx in 0..NUM_AMPS {
        let codes_copy = codes.clone();

        let mem_input = match last_output {
            Some(prev_amp_output) => {
                prev_amp_output
            }
            None => {
                feedback_output.clone()
            }
        };

        {
            let mut q = mem_input.as_ref().borrow_mut();

            if amp_idx == 0 {
                q.push_back(0);
            }
        }

        let mem_output = if amp_idx != (NUM_AMPS-1) {
            Rc::new(RefCell::new(VecDeque::new()))
        } else {
            feedback_output.clone()
        };

//        println!("Interpreter created : {} ... {}",
//            amp_idx,
//            mem_input.as_ref().borrow().front().unwrap()
//        );

        let amp = Interpreter::new(
            0,
            0,
            codes_copy,
            mem_input,
            mem_output.clone(),
        );

        last_output = Some(mem_output);

        amps.push(amp);
    }

    let mut watchdog = 500;
    // process loop
    loop {
        for amp_idx in 0..NUM_AMPS {
            let pass_to_next = amps[amp_idx as usize].process();

            // get result
            if !pass_to_next && amp_idx == (NUM_AMPS-1) {
                return amps[amp_idx as usize].get_last_output();
            }

            watchdog -= 1;

            if (watchdog) < 0 {
                panic!("Reached max loop limit");
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_rel_1() {
        let result = process(
            String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
        );

        assert_eq!(result, 139629729)
    }
}


