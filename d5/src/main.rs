use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

mod interpreter;

fn main() -> () {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let mut codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();

    println!("{:?}", &codes);

//    q1(&mut codes);
    q2(&mut codes);
}

fn q1(codes: &mut [i64]) {
    let mut input_queue = VecDeque::new();
    input_queue.push_front(1);

    interpreter::traverse(codes, &mut input_queue).unwrap()
}

fn q2(codes: &mut [i64]) {
    let mut input_queue = VecDeque::new();
    input_queue.push_front(5);

    interpreter::traverse(codes, &mut input_queue).unwrap()
}