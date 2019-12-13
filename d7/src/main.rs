use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

mod interpreter;

type PhaseCandidates = Vec<i64>;
struct PhaseSelection(i64, PhaseCandidates);
type PhaseSelector = Box<dyn Fn(&Vec<i64>) -> Vec<PhaseSelection>>;

fn phase_selector_default() -> PhaseSelector {
    Box::new(|remaining:&Vec<i64>| {
        (0..remaining.len()).into_iter().map(|idx| {
            let selection = remaining[idx];
            let mut remainder = remaining.clone();
            remainder.remove(idx);

            PhaseSelection(selection, remainder)
        }).collect()
    })
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt").unwrap()
        .read_to_string(&mut input).unwrap();

    let result = process(
        input,
        phase_selector_default()
    );

    println!("Result : {}", result);
}

fn process(input: String, phase_selector: PhaseSelector) -> i64 {
    let codes: Vec<i64> = input
        .split(",")
        .map(|code_txt| code_txt.parse::<i64>().unwrap())
        .collect();


//    let mut input_queues : Vec<VecDeque<i64>> = (0..5).map(|_| VecDeque::new()).collect();
//    let mut output_queues : Vec<VecDeque<i64>> = (0..5).map(|_| VecDeque::new()).collect();

    try_phase_candidates(&codes, phase_selector)
}

fn try_phase_candidates(codes: &[i64], phase_selector: PhaseSelector) -> i64 {
    let mut input_0= VecDeque::new();
    input_0.push_front(0);
    try_phase_candidate(
        codes, 1, input_0, &phase_selector, &vec![0,1,2,3,4]
    )
}

// todo : generic const for amp_num <-> output_of_prev ?

const NUM_AMPS:i64 = 5;

fn try_phase_candidate(
    codes: &[i64], amp_number:i64, output_of_previous:VecDeque<i64>,
    phase_selector: &PhaseSelector, phase_remaining: &Vec<i64>
) -> i64 {
    if amp_number == NUM_AMPS + 1 {
        return output_of_previous.front().unwrap().clone();
    }

    let mut return_values = Vec::new();

    for PhaseSelection(phase_candidate, next_phase_remaining) in phase_selector(phase_remaining).iter() {
        let mut codes_copy = Vec::from(codes);

        let mut input_queue = VecDeque::new();
        let mut output_queue = VecDeque::new();

        input_queue.push_front(*phase_candidate);
        input_queue.push_back(output_of_previous.front().unwrap().clone());

        interpreter::traverse(
            &mut codes_copy,
            &mut input_queue, &mut output_queue
        ).unwrap();

        return_values.push(
            try_phase_candidate(
                codes, amp_number + 1, output_queue,
                phase_selector, next_phase_remaining
            )
        )
    }

//    assert_eq!(return_values.len(), 4);

    return_values.iter().cloned().fold(None, |acc, item| {
        match acc {
            None => Some(item),
            Some(prev_max) => Some(i64::max(prev_max, item))
        }
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn phase_selector_test(main_source: Vec<i64>) -> PhaseSelector {
        let idx : Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
        Box::new(move |_| {
            let selection = main_source[*idx.borrow()];
//            let mut rem_copy -= main_source[idx+1..].clone();
//            let mut rem_copy = main_source.clone();

            *idx.borrow_mut() += 1;

            vec![PhaseSelection(selection, vec![])]
        })
    }

    #[test]
    fn test_1() {
        let result = process(
            String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            phase_selector_test(vec![4,3,2,1,0])
        );

        assert_eq!(result, 43210)
    }

    #[test]
    fn test_2() {
        let result = process(
            String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        phase_selector_test(vec![0,1,2,3,4])
        );

        assert_eq!(result, 54321)
    }

    #[test]
    fn test_3() {
        let result = process(
            String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
            phase_selector_test(vec![1,0,4,3,2])
        );

        assert_eq!(result, 65210)
    }

}


