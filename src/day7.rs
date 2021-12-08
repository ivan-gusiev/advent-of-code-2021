use crate::util::{self, BResult};

type Input = [i32];

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day7.txt", "input/day7-test.txt")?;
    let input = util::split_parse::<i32>(&input_string, regex::Regex::new(",")?)?;

    part1(&input);
    part2(&input);
    part3(&input);

    Ok(())
}

fn part1(input: &Input) {
    println!(
        "part 1 solution: {}",
        generic_min_distance(input, |x, target| (x - target).abs())
    );
}

fn part2(input: &Input) {
    fn sqr_dist(x1: i32, x2: i32) -> i32 {
        let abs = (x2 - x1).abs();
        abs * (abs + 1) / 2
    }

    println!("part 2 solution: {}", generic_min_distance(input, sqr_dist));
}

fn part3(input: &Input) {
    fn mad_dist(x1: i32, x2: i32) -> i32 {
        (x2 - x1).abs()
    }

    fn sqr_dist(x1: i32, x2: i32) -> i32 {
        let abs = (x2 - x1).abs();
        abs * (abs + 1) / 2
    }

    let (min_mad, _) = generic_min_distance_value(input, mad_dist);
    let (min_sqr, _) = generic_min_distance_value(input, sqr_dist);
    let mean = (input.iter().sum::<i32>() as f64) / input.len() as f64;
    let median = util::median(&mut input.iter().map(|x| *x).collect::<Vec<i32>>());

    dbg!(min_mad, min_sqr, mean, median);
}

fn generic_min_distance_value(input: &Input, distance_fn: impl Fn(i32, i32) -> i32) -> (i32, i32) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let distances: Vec<(i32, i32)> = (min..=max)
        .map(|target| (target, input.iter().map(|x| distance_fn(*x, target)).sum()))
        .collect();

    *distances.iter().min_by_key(|(_, x)| x).unwrap()
}

fn generic_min_distance(input: &Input, distance_fn: impl Fn(i32, i32) -> i32) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let distances: Vec<i32> = (min..=max)
        .map(|target| input.iter().map(|x| distance_fn(*x, target)).sum())
        .collect();

    *distances.iter().min().unwrap()
}
