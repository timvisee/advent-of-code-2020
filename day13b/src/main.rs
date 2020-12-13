pub fn main() {
    let lines: Vec<(i64, i64)> = include_str!("../input.txt")
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|l| (l - i as i64, l)))
        .collect();
    println!("{}", crt(&lines));
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

fn crt(remo: &[(i64, i64)]) -> i64 {
    let prod = remo.iter().map(|n| n.1).product::<i64>();
    remo.iter()
        .map(|(re, mo)| {
            let p = prod / mo;
            re * ((egcd(p, *mo).1 % mo + mo) % mo) * p
        })
        .sum::<i64>()
        % prod
}
