use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

mod lib;

use lib::interpreter::InterpreterProcessResult;

const NUM_AMPS: i64 = 1;

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let mut runner = lib::runner::Runner::initialize(
        input
    );

    let mut oc = 0;
    let mut watchdog = 80000;
    loop {
        let pass_to_next = runner.execute_next_loop();

//        println!("=====================");

        watchdog -= 1;

        if (watchdog) < 0 {
            runner.robot.visualize_visited_coords();
            panic!("Reached max loop limit");
        }

        if let Some(rv) = pass_to_next {
            runner.robot.visualize_visited_coords();
            println!("Dist : {}", rv);

            break;
        }
    }


//    panic!("use browser");
//    println!("{}", result);
}