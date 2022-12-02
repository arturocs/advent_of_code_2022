fn main() {
    let input = include_str!("../input");

    let mut calories_by_elf: Vec<i32> = input
        .split("\n\n")
        .map(|i| i.lines().map(|j| j.parse::<i32>().unwrap()).sum())
        .collect();

    calories_by_elf.sort_unstable_by(|a, b| b.cmp(a));

    let max_calories = calories_by_elf.first().unwrap();
    println!("Part 1: {max_calories}");

    let top_three_sum: i32 = calories_by_elf[..3].iter().sum();
    println!("Part 2 {top_three_sum:?}");
}
