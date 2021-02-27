use std::mem;

pub fn main() {
    let mut program: Vec<(&[u8], i32)> = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|ins| (&ins[0..3], atoi::atoi(&ins[4..]).unwrap()))
        .collect();

    let mut other_op: &[u8] = b"nop";
    for pc in run(&program).0 {
        if program[pc].0 == b"jmp" {
            mem::swap(&mut program[pc].0, &mut other_op);
            if let (_, Some(acc)) = run(&program) {
                println!("{}", acc);
                break;
            }
            mem::swap(&mut program[pc].0, &mut other_op);
        }
    }
}

/// Run program. Return `Some(acc)` on success, `None` on infinite loop.
#[inline(always)]
fn run(program: &[(&[u8], i32)]) -> (Vec<usize>, Option<i32>) {
    let (mut visited, mut pc, mut acc) = (Vec::with_capacity(64), 0, 0);
    while !visited.contains(&pc) {
        visited.push(pc);
        match program.get(pc) {
            Some((b"acc", val)) => acc += val,
            Some((b"jmp", val)) => pc += *val as usize - 1,
            None => return (visited, Some(acc)),
            _ => {}
        }
        pc += 1;
    }
    (visited, None)
}
