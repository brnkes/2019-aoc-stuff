use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::interpreter::Interpreter;

mod interpreter;

const NUM_AMPS: i64 = 5;

fn phase_combinations(remaining_src: &Vec<i64>) -> Vec<Vec<i64>> {
    (0..remaining_src.len())
        .into_iter()
        .map(|idx| {
            let selection = remaining_src[idx];
            let mut remainder = remaining_src.clone();
            remainder.remove(idx);

            if remainder.len() > 0 {
                let result: Vec<Vec<i64>> = phase_combinations(&remainder)
                    .iter()
                    .map(|child| [&[selection][..], child].concat())
                    .collect();

                return result;
            } else {
                return vec![vec![selection]];
            }
        })
        .flatten()
        .collect()
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let result = process(
        input,
        &phase_combinations(&vec![5, 6, 7, 8, 9]),
    );

    println!("Result : {}", result);
}

fn process(input: String, phases: &Vec<Vec<i64>>) -> i64 {
    let mut largest = 0;
    for phase in phases {
        let trial_result = generate_and_run_amps(&input, phase);
        largest = max(largest, trial_result);
    }

    largest
}

fn generate_and_run_amps(input: &str, phase: &Vec<i64>) -> i64 {
    let codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();

    let feedback_output = Rc::new(RefCell::new(VecDeque::new()));
    let mut last_output = None;
    let mut amps: Vec<Interpreter> = Vec::new();

    for amp_idx in 0..NUM_AMPS {
        let codes_copy = codes.clone().into_boxed_slice();

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
            q.push_front(phase[(amp_idx) as usize]);
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
    fn test_phase_combinations() {
        let res = phase_combinations(&vec![0,1,2]);
        println!("{:?}", res);

        assert_eq!(res.len(),6);
    }

    #[test]
    fn test_1() {
        let result = process(
            String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            &vec![vec![4,3,2,1,0]]
        );

        assert_eq!(result, 43210)
    }

    #[test]
    fn test_2() {
        let result = process(
            String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            &vec![vec![0,1,2,3,4]]
        );

        assert_eq!(result, 54321)
    }

    #[test]
    fn test_3() {
        let result = process(
            String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
            &vec![vec![1,0,4,3,2]]
        );

        assert_eq!(result, 65210)
    }

    #[test]
    fn test_loop_1() {
        let result = process(
            String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            &vec![vec![9, 8, 7, 6, 5]],
        );

        assert_eq!(result, 139629729)
    }

    #[test]
    fn test_loop_2() {
        let result = process(
            String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            &vec![vec![9, 7, 8, 5, 6]],
        );

        assert_eq!(result, 18216)
    }


}


