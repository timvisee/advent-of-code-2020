use std::mem::MaybeUninit;

pub fn main() {
    let cups: Vec<u32> = include_bytes!("../input.txt")
        .into_iter()
        .filter(|&b| b != &b'\n')
        .map(|b| (b - b'0') as u32)
        .collect();

    // Build cup index linked list up to 1000000
    let mut indices: [u32; 1_000_001] = unsafe { MaybeUninit::uninit().assume_init() };
    let last = cups[1..].iter().fold(cups[0], |prev, cup| {
        indices[prev as usize] = *cup;
        *cup
    });
    (cups.len() + 1..1_000_000).for_each(|c| indices[c] = c as u32 + 1);
    indices[last as usize] = cups.len() as u32 + 1;
    indices[1_000_000] = cups[0];

    (0..10_000_000).fold(cups[0], |cur, _| {
        let a = indices[cur as usize];
        let b = indices[a as usize];
        let c = indices[b as usize];

        let dest = (1..cur)
            .rev()
            .filter(|&n| n != a && n != b && n != c)
            .next()
            .unwrap_or_else(|| {
                (cur + 1..=1_000_000)
                    .rev()
                    .filter(|&n| n != a && n != b && n != c)
                    .next()
                    .unwrap()
            });

        indices[cur as usize] = indices[c as usize];
        indices[c as usize] = indices[dest as usize];
        indices[dest as usize] = a;
        indices[cur as usize]
    });

    let a = indices[1] as u64;
    let b = indices[a as usize] as u64;
    println!("{}", a * b);
}
