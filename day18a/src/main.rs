use nom::*;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter(|b| !b.is_empty())
            .map(|e| expr(e).unwrap().1)
            .sum::<usize>()
    );
}

named!(
    digit<usize>,
    map!(
        verify!(take!(1), |d: &[u8]| nom::character::is_digit(d[0])),
        |d| (d[0] - b'0') as usize
    )
);

named!(
    par<usize>,
    alt!(delimited!(tag!("("), expr, tag!(")")) | digit)
);

named!(
    expr<usize>,
    do_parse!(
        first: par
            >> sum: fold_many1!(
                complete!(pair!(delimited!(tag!(" "), one_of!("+*"), tag!(" ")), par)),
                first,
                |acc, (op, val)| match op {
                    '*' => acc * val,
                    '+' => acc + val,
                    _ => unreachable!(),
                }
            )
            >> (sum)
    )
);
