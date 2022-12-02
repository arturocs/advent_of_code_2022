use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => return Err("Unknown shape".into()),
        };
        Ok(shape)
    }
}
#[derive(Clone, Copy)]
enum MatchResult {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

impl FromStr for MatchResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "X" => MatchResult::Lost,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => return Err("Unknown result".into()),
        };
        Ok(shape)
    }
}

fn play_match(me: Shape, opponent: Shape) -> MatchResult {
    match (me, opponent) {
        (Shape::Rock, Shape::Rock) => MatchResult::Draw,
        (Shape::Rock, Shape::Paper) => MatchResult::Lost,
        (Shape::Rock, Shape::Scissors) => MatchResult::Win,
        (Shape::Paper, Shape::Rock) => MatchResult::Win,
        (Shape::Paper, Shape::Paper) => MatchResult::Draw,
        (Shape::Paper, Shape::Scissors) => MatchResult::Lost,
        (Shape::Scissors, Shape::Rock) => MatchResult::Lost,
        (Shape::Scissors, Shape::Paper) => MatchResult::Win,
        (Shape::Scissors, Shape::Scissors) => MatchResult::Draw,
    }
}

fn play_reverse_match(result: MatchResult, opponent: Shape) -> Shape {
    match (result, opponent) {
        (MatchResult::Win, Shape::Rock) => Shape::Paper,
        (MatchResult::Win, Shape::Paper) => Shape::Scissors,
        (MatchResult::Win, Shape::Scissors) => Shape::Rock,
        (MatchResult::Draw, Shape::Rock) => Shape::Rock,
        (MatchResult::Draw, Shape::Paper) => Shape::Paper,
        (MatchResult::Draw, Shape::Scissors) => Shape::Scissors,
        (MatchResult::Lost, Shape::Rock) => Shape::Scissors,
        (MatchResult::Lost, Shape::Paper) => Shape::Rock,
        (MatchResult::Lost, Shape::Scissors) => Shape::Paper,
    }
}

fn main() {
    let input = include_str!("../input");
    let total_points_1: i32 = input
        .lines()
        .map(|l| {
            let mut a = l.split_whitespace();
            let opponent = Shape::from_str(a.next().unwrap()).unwrap();
            let me = Shape::from_str(a.next().unwrap()).unwrap();
            play_match(me, opponent) as i32 + me as i32
        })
        .sum();
    println!("Part 1: {total_points_1}");

    let total_points_2: i32 = input
        .lines()
        .map(|l| {
            let mut a = l.split_whitespace();
            let opponent = Shape::from_str(a.next().unwrap()).unwrap();
            let result = MatchResult::from_str(a.next().unwrap()).unwrap();
            play_reverse_match(result, opponent) as i32 + result as i32
        })
        .sum();
    println!("Part 2: {total_points_2}");
}
