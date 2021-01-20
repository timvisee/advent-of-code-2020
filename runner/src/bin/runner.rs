use took::Timer;

fn main() {
    let timer = Timer::new();
    runner::jobs().into_iter().for_each(|j| j.0());
    timer.took().describe("everything");
}
