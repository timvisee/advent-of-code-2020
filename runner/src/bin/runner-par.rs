use rayon::prelude::*;

fn main() {
    // Build threadpool with larger stack size
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(12_800_000)
        .build_global()
        .unwrap();

    let jobs = runner::jobs();
    (0..jobs.len()).into_par_iter().for_each(|i| jobs[i]());
}
