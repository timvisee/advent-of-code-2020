use std::thread;

fn main() {
    runner::jobs()
        .into_iter()
        .map(|j| thread::spawn(j))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|t| t.join().unwrap());
}
