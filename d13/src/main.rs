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

//    File::open("./input-q2.txt").unwrap()
//        .read_to_string(&mut input).unwrap();

    let mut game = lib::Game::initialize(
        input
    );

    let mut oc = 0;
    let mut watchdog = 50000;
    loop {
        let pass_to_next = game.loop_once();

        match pass_to_next {
            InterpreterProcessResult::ThreeOutputs => {
                oc += 1;
                //            assert_eq!(mem_output.borrow().len(), 3, "Should've outputted 3 values.");
            },
            InterpreterProcessResult::WaitingOnInput => {
                println!("Outputs so far : {}", oc);
                oc = 0;
            },
            InterpreterProcessResult::Ended => { break; },
        }

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

    println!("Outputs last : {}", oc);

//    panic!("use browser");
//    println!("{}", result);
}