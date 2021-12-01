use crate::util::{parse_lines, read_file, BResult};

pub fn run() -> BResult<()> {
    let input = read_file("input/day1.txt")?;
    let nums: Vec<i32> = parse_lines(&input)?;

    part1(&nums);
    part2(&nums);

    Ok(())
}

fn part1(nums: &[i32]) {
    let result = nums.array_windows::<2>().filter(less).count();

    println!("part 1 result: {}", result);
}

fn part2(nums: &[i32]) {
    let result = nums
        .array_windows::<3>()
        .map(|arr| arr.iter().sum())
        .collect::<Vec<i32>>() // sums for windows
        .array_windows::<2>()
        .filter(less)
        .count();

    println!("part 2 result: {}", result);
}

fn less(arr: &&[i32; 2]) -> bool {
    PartialOrd::lt(&arr[0], &arr[1])
}
