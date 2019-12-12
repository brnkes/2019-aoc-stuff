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

    q1(&mut codes);
}

fn q1(codes: &mut [i64]) {
    let mut input_queue = VecDeque::new();
    input_queue.push_front(1);

    interpreter::traverse(codes, &mut input_queue).unwrap()
}

/*
fn q2(codes: &[i64]) -> Option<i64> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut copied = codes.to_vec();
            copied[1] = noun;
            copied[2] = verb;
            interpreter::traverse(&mut copied).unwrap();

            if copied[0] == 19690720 {
                return Some(100 * noun + verb)
            }
        }
    }

    None
}*/
