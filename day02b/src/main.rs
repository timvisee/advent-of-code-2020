#[macro_use]
extern crate nom;

use atoi::atoi;
use nom::character::complete::alpha1;

fn main() {
    println!(
        "{}",
        entries(&std::fs::read("./input.txt").unwrap())
            .unwrap()
            .1
            .into_iter()
            .filter(|p| *p)
            .count()
    );
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));

named!(minmax<&[u8], (usize, usize)>, separated_pair!(usize, char!('-'), usize));

named!(needle<&[u8], u8>, terminated!(nom::number::complete::u8, char!(':')));

named!(entry<&[u8], (usize, usize, u8, &[u8])>,
    do_parse!(
        mm: minmax >>
        char!(' ') >>
        needle: needle >>
        char!(' ') >>
        haystack: alpha1 >>
        (mm.0, mm.1, needle, haystack)
    )
);

named!(test_entry<&[u8], bool>, map!(entry, |e| ((e.3[e.0 - 1] == e.2) != (e.3[e.1 - 1] == e.2))));

named!(entries<&[u8], Vec<bool>>, separated_list1!(tag!("\n"), test_entry));
