use std::fs::File;
use std::io::Read;

mod interpreter;

fn main() -> () {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let mut codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();

//    q1(&mut codes);
//    print!("{}", codes[0]);

    print!("{}",q2(&codes).unwrap());
}

fn q1(codes: &mut [i64]) {
    codes[1] = 12;
    codes[2] = 2;

    // todo : try out custom error wrapper/crate
    interpreter::traverse(codes).unwrap();
}

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
}