pub fn main() {
    let (moduli, residues) = include_str!("../input.txt")
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse::<i64>().ok().map(|l| (i, l)))
        .fold((Vec::new(), Vec::new()), |(mut mo, mut re), (i, l)| {
            mo.push(l);
            re.push(l - i as i64);
            (mo, re)
        });
    println!("{}", chinese_remainder(&residues, &moduli));
}

#[inline(always)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn chinese_remainder(re: &[i64], mo: &[i64]) -> i64 {
    let prod = mo.iter().product::<i64>();
    re.iter()
        .zip(mo)
        .map(|(&re, &mo)| {
            let p = prod / mo;
            re * ((egcd(p, mo).1 % mo + mo) % mo) * p
        })
        .sum::<i64>()
        % prod
}
