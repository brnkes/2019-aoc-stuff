use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::interpreter::Interpreter;

mod interpreter;
mod world;

const NUM_AMPS: i64 = 1;

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let q1_i = Some(1);
    let q2_i = Some(2);

    let result = process(
        input,
        q2_i
    );

    println!("Result : {:?}", result);
}

fn process(input: String, pass_input: Option<i64>) -> VecDeque<i64> {
    generate_and_run_amps(&input, pass_input)
}

fn generate_and_run_amps(input: &str, pass_input: Option<i64>) -> VecDeque<i64> {
    let codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();

    let codes_copy = codes.clone();

    let mem_input = Rc::new(RefCell::new(VecDeque::new()));

    {
        let mut q = mem_input.as_ref().borrow_mut();
        if let Some(initial_input) = pass_input {
            q.push_back(initial_input);
        }
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
        mem_input,
        mem_output.clone(),
    );


    let mut watchdog = 500;
    // process loop
    loop {
        let pass_to_next = amp.process();

        // get result
        if !pass_to_next {
            return mem_output.as_ref().borrow().clone();
        }

        watchdog -= 1;

        if (watchdog) < 0 {
            panic!("Reached max loop limit");
        }
    }
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
            None
        );

        let string_repr = result
            .iter()
            .fold(None, |acc, next| {
                match acc {
                    None => Some(format!("{}", next)),
                    Some(prev) => Some(format!("{},{}", prev, next))
                }
            })
            .unwrap();

        assert_eq!(string_repr, "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    }

    #[test]
    fn test_rel_2() {
        let result = process(
            String::from("1102,34915192,34915192,7,4,7,99,0"),
            Some(0)
        );

        assert_eq!(result[0], 1219070632396864)
    }

    #[test]
    fn test_rel_3() {
        let result = process(
            String::from("104,1125899906842624,99"),
            Some(0)
        );

        assert_eq!(result[0], 1125899906842624)
    }
}


