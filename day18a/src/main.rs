#![feature(custom_inner_attributes)]#![rustfmt::skip]

use nom::{character::is_digit, *};

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .map(|e| expr(e).unwrap().1)
            .sum::<usize>()
    );
}

named!(digit<usize>, map!(take_while_m_n!(1, 1, is_digit), |d| (d[0] - b'0') as usize));
named!(unit<usize>, alt!(delimited!(tag!("("), expr, tag!(")")) | digit));

named!(
    expr<usize>,
    do_parse!(
        first: unit
            >> sum: fold_many1!(
                complete!(pair!(delimited!(tag!(" "), one_of!("+*"), tag!(" ")), unit)),
                first,
                |acc, (op, num)| match op {
                    '+' => acc + num,
                    '*' => acc * num,
                    _ => unreachable!(),
                })
            >> (sum)
    )
);
