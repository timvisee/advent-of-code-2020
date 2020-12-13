pub fn main() {
    let (est, lines) = include_str!("../input.txt").split_once('\n').unwrap();
    let est = est.parse::<usize>().unwrap();
    let (line, wait) = lines
        .split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .map(|l| (l, l - (est % l)))
        .min_by_key(|l| l.1)
        .unwrap();
    println!("{}", line * wait);
}
