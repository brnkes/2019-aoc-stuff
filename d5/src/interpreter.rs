enum Op {
    Sum,
    Mul
}

fn parse_opcode(op_code:i64) -> Result<Op,String> {
    match op_code {
        1 => Ok(Op::Sum),
        2 => Ok(Op::Mul),
        _ => Err(String::from("Wrong opcode"))
    }
}

fn get_op(code:Op) -> Box<dyn Fn(i64,i64) -> i64> {
    match code {
        Op::Sum => {
            Box::new(|arg1, arg2| arg1 + arg2)
        }
        Op::Mul => {
            Box::new(|arg1, arg2| arg1 * arg2)
        },
    }
}

fn eval(op_code:i64, arg1:i64, arg2:i64) -> Result<i64,String> {
    let code = parse_opcode(op_code)?;
    Ok(get_op(code)(arg1, arg2))
}

pub fn traverse(program:&mut [i64]) -> Result<(),String> {
    let mut exec_ptr = 0;
    while exec_ptr < program.len() {
        if program[exec_ptr] == 99 {
            return Ok(());
        }

        program[program[exec_ptr+3] as usize] = eval(
            program[exec_ptr],
            program[program[exec_ptr+1] as usize],
            program[program[exec_ptr+2] as usize]
        )?;
        exec_ptr+=4;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_sum() {
        let result = eval(1,2,12);
        assert_eq!(result.unwrap(), 14)
    }

    #[test]
    fn test_eval_mul() {
        let result = eval(2,2,12);
        assert_eq!(result.unwrap(), 24)
    }

    #[test]
    fn test_traverse() {
        let mut program = [1,9,10,3,2,3,11,0,99,30,40,50];
        traverse(&mut program);

        print!("{:?}", program);
    }
}

