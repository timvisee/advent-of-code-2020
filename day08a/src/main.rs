fn main() {
    let program: Vec<(&str, isize)> = include_str!("../input.txt")
        .lines()
        .map(|ins| {
            let mut sp = ins.splitn(2, ' ');
            (sp.next().unwrap(), sp.next().unwrap().parse().unwrap())
        })
        .collect();

    let (mut covered, mut pc, mut acc) = (vec![], 0, 0);

    while !covered.contains(&pc) {
        covered.push(pc);
        match program[pc] {
            ("acc", val) => acc += val,
            ("jmp", val) => pc += val as usize - 1,
            _ => {}
        }
        pc += 1;
    }

    println!("{}", acc);
}
