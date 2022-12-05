const WIDTH: usize = 4;

fn tranpose(m: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let mut tm = vec![vec![""; m.len() - 1]; m[0].len()];
    for i in 0..m.len() - 1 {
        for j in 0..m[0].len() {
            tm[j][i] = m[i][j];
        }
    }
    tm
}

fn mirror_and_filter(m: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    m.into_iter()
        .map(|i| i.into_iter().rev().filter(|j| !j.is_empty()).collect())
        .collect()
}

fn parse_crate_line(l: &str) -> Vec<&str> {
    (WIDTH..=l.len())
        .step_by(WIDTH)
        .map(|i| {
            let s = l[i - WIDTH..i].trim();
            s.is_empty()
                .then_some("")
                .unwrap_or_else(|| s.trim_matches(&['[', ']'] as &[_]))
        })
        .collect()
}

fn parse_crates(crates: &str) -> Vec<Vec<&str>> {
    let crates: Vec<Vec<_>> = crates[..crates.len() - 1]
        .split_inclusive("\n")
        .map(parse_crate_line)
        .collect();
    mirror_and_filter(tranpose(crates))
}

fn parse_moves(moves: &str) -> Vec<(usize, usize, usize)> {
    moves
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace().skip(1).step_by(2);
            let ammount = it.next().unwrap().parse().unwrap();
            let origin: usize = it.next().unwrap().parse().unwrap();
            let destination: usize = it.next().unwrap().parse().unwrap();
            (ammount, origin - 1, destination - 1)
        })
        .collect()
}

fn crane_mover_9000<'a>(
    mut crates: Vec<Vec<&'a str>>,
    moves: &'a [(usize, usize, usize)],
) -> Vec<Vec<&'a str>> {
    for &(ammount, origin, destination) in moves {
        for _ in 0..ammount {
            let c = crates[origin].pop().unwrap();
            crates[destination].push(c);
        }
    }
    crates
}

fn crane_mover_9001<'a>(
    mut crates: Vec<Vec<&'a str>>,
    moves: &'a [(usize, usize, usize)],
) -> Vec<Vec<&'a str>> {
    for &(ammount, origin, destination) in moves {
        let moved_crates: Vec<_> = (0..ammount)
            .map(|_| crates[origin].pop().unwrap())
            .collect();
        crates[destination].extend(moved_crates.into_iter().rev())
    }
    crates
}

fn main() {
    let input = include_str!("../input");
    let mut it = input.split_inclusive("\n\n");
    let crates = it.next().unwrap();
    let moves = it.next().unwrap();
    let parsed_crates = parse_crates(crates);
    let parsed_moves = parse_moves(moves);

    let part_1: String = crane_mover_9000(parsed_crates.clone(), &parsed_moves)
        .into_iter()
        .map(|i| *i.last().unwrap())
        .collect();

    println!("Part 1: {part_1}");

    let part_2: String = crane_mover_9001(parsed_crates, &parsed_moves)
        .into_iter()
        .map(|i| *i.last().unwrap())
        .collect();

    println!("Part 2: {part_2}");
}
