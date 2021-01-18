fn main() {
    let mut cups: Vec<u8> = include_bytes!("../input.txt")
        .into_iter()
        .filter(|&b| b != &b'\n')
        .map(|b| b - b'0')
        .collect();
    let highest = *cups.iter().max().unwrap();

    for _ in 0..100 {
        let cur = cups[0];
        let removed = [cups.remove(1), cups.remove(1), cups.remove(1)];
        for target in (1..cur).rev().chain((cur..=highest).rev()) {
            if let Some(pos) = cups.iter().position(|&c| c == target) {
                for removed in removed.iter().rev() {
                    cups.insert(pos + 1, *removed);
                }
                break;
            }
        }

        cups.rotate_left(1);
    }

    let offset = cups.iter().position(|&c| c == 1).unwrap();
    cups.rotate_left(offset);
    println!(
        "{}",
        cups.iter()
            .skip(1)
            .map(|c| (c + b'0') as char)
            .collect::<String>()
    );
}
