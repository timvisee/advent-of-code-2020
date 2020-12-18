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

named!(par<usize>, delimited!(tag!("("), expr, tag!(")")));

named!(par_digit<usize>, alt!(digit | par));

named!(
    expr<usize>,
    do_parse!(
        first: par_digit
            >> sum: fold_many1!(
                complete!(pair!(
                    delimited!(tag!(" "), one_of!("+*"), tag!(" ")),
                    par_digit
                )),
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
