fn main() {
    let mut program: Vec<(&str, isize)> = include_str!("../input.txt")
        .lines()
        .map(|ins| {
            let mut sp = ins.splitn(2, ' ');
            (sp.next().unwrap(), sp.next().unwrap().parse().unwrap())
        })
        .collect();
    program.push(("don", 0));

    for i in 0..program.len() {
        swap_jmp_nop(&mut program[i]);
        if let Some(acc) = run(&program) {
            eprintln!("{}", acc);
            return;
        }
        swap_jmp_nop(&mut program[i]);
    }
}

fn swap_jmp_nop(instr: &mut (&str, isize)) {
    match instr {
        ("nop", _) => instr.0 = "jmp",
        ("jmp", _) => instr.0 = "nop",
        _ => {}
    }
}

/// Run program. Return `Some(acc)` on success, `None` on infinite loop.
fn run(program: &[(&str, isize)]) -> Option<isize> {
    let (mut covered, mut pc, mut acc) = (vec![], 0, 0);
    while !covered.contains(&pc) {
        covered.push(pc);
        match program[pc] {
            ("acc", val) => acc += val,
            ("jmp", val) => pc += val as usize - 1,
            ("don", _) => return Some(acc),
            _ => {}
        }
        pc += 1;
    }
    None
}
