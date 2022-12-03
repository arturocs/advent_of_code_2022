use std::collections::BTreeSet;
fn calculate_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => unreachable!(),
    }
}
fn main() {
    let input = include_str!("../input");
    let part_1: i32 = input
        .lines()
        .map(|s| {
            let a = BTreeSet::from_iter(s[..s.len() / 2].chars());
            let b = BTreeSet::from_iter(s[s.len() / 2..].chars());
            a.intersection(&b)
                .cloned()
                .map(calculate_priority)
                .sum::<i32>()
        })
        .sum();
    println!("Part 1: {part_1}");

    let part_2: i32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|s| {
            let a = BTreeSet::from_iter(s[0].chars());
            let b = BTreeSet::from_iter(s[1].chars());
            let c = BTreeSet::from_iter(s[2].chars());
            let intersection = &(&a & &b) & &c;
            intersection.into_iter().map(calculate_priority)
        })
        .sum();
    println!("Part 2: {part_2}");
}
