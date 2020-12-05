fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|l| row(l) * 8 + col(l))
            .max()
            .unwrap(),
    );
}

fn row(line: &str) -> usize {
    let mut pos = 0;

    for (i, a) in line.bytes().take(7).enumerate() {
        if a == b'B' {
            pos |= 1 << (7 - i - 1);
        }
    }

    pos
}

fn col(line: &str) -> usize {
    let mut pos = 0;

    for (i, a) in line.bytes().skip(7).take(3).enumerate() {
        if a == b'R' {
            pos |= 1 << (3 - i - 1);
        }
    }

    pos
}
