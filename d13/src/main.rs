use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

mod lib;

const NUM_AMPS: i64 = 1;

fn main() {
    let mut input = String::new();

//    File::open("./input.txt").unwrap()
//        .read_to_string(&mut input).unwrap();

    File::open("./input-q2.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let result = lib::process(
        input
    );

    println!("{}", result);
}