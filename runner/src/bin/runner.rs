fn main() {
    runner::jobs().into_iter().for_each(|j| j());
}
