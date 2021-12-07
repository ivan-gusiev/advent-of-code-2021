use crate::util::{read_file_maybe_test, split_parse, BResult};

type Input = [i32];

pub fn run() -> BResult<()> {
    let input_string = read_file_maybe_test("input/day7.txt", "input/day7-test.txt")?;
    let input = split_parse::<i32>(&input_string, regex::Regex::new(",")?)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Input) {
    println!("part 1 solution: {}", generic_min_distance(input, |x, target| (x - target).abs()));
}

fn part2(input: &Input) {
    fn sqr_dist(x1: i32, x2: i32) -> i32 {
        let abs = (x2 - x1).abs();
        abs * (abs + 1) / 2
    }

    println!("part 2 solution: {}", generic_min_distance(input, sqr_dist));
}

fn generic_min_distance(input: &Input, distance_fn: impl Fn(i32, i32) -> i32) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let distances: Vec<i32> = (min..=max)
        .map(|target| input.iter().map(|x| distance_fn(*x, target)).sum())
        .collect();

    *distances.iter().min().unwrap()
}