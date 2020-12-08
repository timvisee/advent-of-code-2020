use rayon::prelude::*;

fn main() {
    let jobs = runner::jobs();
    (0..jobs.len()).into_par_iter().for_each(|i| jobs[i]());
}
