fn is_contained(e1: (i32, i32), e2: (i32, i32)) -> bool {
    (e1.0 <= e2.0 && e1.1 >= e2.1) || (e2.0 <= e1.0 && e2.1 >= e1.1)
}

fn is_overlaping(e1: (i32, i32), e2: (i32, i32)) -> bool {
    e1.0 <= e2.1 && e2.0 <= e1.1
}

fn parse_sections(s: &str) -> (i32, i32) {
    let mut it = s.split('-').map(|i| i.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}

fn main() {
    let input = include_str!("../input");
    let parsed_input: Vec<_> = input
        .lines()
        .map(|s| {
            let mut it = s.split(',');
            let elf1 = it.next().unwrap();
            let elf2 = it.next().unwrap();
            (parse_sections(elf1), parse_sections(elf2))
        })
        .collect();

    let part_1 = parsed_input
        .iter()
        .filter(|&&(e1, e2)| is_contained(e1, e2))
        .count();
    println!("Part 1: {part_1}");

    let part_2 = parsed_input
        .into_iter()
        .filter(|&(e1, e2)| is_overlaping(e1, e2))
        .count();
    println!("Part 2: {part_2}");
}
