fn main() {
    let program: Vec<(&[u8], isize)> = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|ins| (&ins[0..3], atoi::atoi(&ins[4..]).unwrap()))
        .collect();

    let (mut visited, mut pc, mut acc) = (vec![], 0, 0);
    while !visited.contains(&pc) {
        visited.push(pc);
        match program[pc] {
            (b"acc", val) => acc += val,
            (b"jmp", val) => pc += val as usize - 1,
            _ => {}
        }
        pc += 1;
    }

    println!("{}", acc);
}
