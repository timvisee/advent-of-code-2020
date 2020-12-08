fn main() {
    let mut program: Vec<(&str, isize)> = include_str!("../input.txt")
        .lines()
        .map(|ins| {
            let mut sp = ins.splitn(2, ' ');
            (sp.next().unwrap(), sp.next().unwrap().parse().unwrap())
        })
        .collect();

    let swp_jmp_nop = |ins: &mut _| match ins {
        ("nop", _) => ins.0 = "jmp",
        ("jmp", _) => ins.0 = "nop",
        _ => {}
    };
    for pc in run(&program).0 {
        swp_jmp_nop(&mut program[pc]);
        if let (_, Some(acc)) = run(&program) {
            println!("{}", acc);
            break;
        }
        swp_jmp_nop(&mut program[pc]);
    }
}

/// Run program. Return `Some(acc)` on success, `None` on infinite loop.
#[inline(always)]
fn run(program: &[(&str, isize)]) -> (Vec<usize>, Option<isize>) {
    let (mut visited, mut pc, mut acc) = (Vec::with_capacity(64), 0, 0);
    while !visited.contains(&pc) {
        visited.push(pc);
        match program.get(pc) {
            Some(("acc", val)) => acc += val,
            Some(("jmp", val)) => pc += *val as usize - 1,
            None => return (visited, Some(acc)),
            _ => {}
        }
        pc += 1;
    }
    (visited, None)
}
