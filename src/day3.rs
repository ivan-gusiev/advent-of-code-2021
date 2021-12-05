use crate::util::{parse_lines, read_file, BResult};

pub fn run() -> BResult<()> {
    let input = read_file("input/day3.txt")?;
    let values: Vec<String> = parse_lines(&input)?;

    part1(&values);
    part2(&values);

    Ok(())
}

fn part1(values: &[String]) {
    let total_values = values.len();
    let width = values[0].len();
    let threshold = total_values / 2;

    let mut counts: Vec<usize> = vec![0; width];
    for value in values {
        for i in 0..width {
            let bit: usize = match value.chars().nth(i).unwrap() {
                '1' => 1,
                _ => 0,
            };
            counts[i] += bit;
        }
    }

    let result_number = counts
        .iter()
        .map(|x| if x > &threshold { "1" } else { "0" })
        .collect::<String>();
    println!("counts: {:?}", counts);
    println!("number: {}", result_number);

    let gamma = parse_binary(&result_number);
    let epsilon = !gamma & ((2usize).pow((width) as u32) - 1);

    println!("gamma: {} epsilon: {}", gamma, epsilon);
    println!("part 1 result: {}", gamma * epsilon);
}

fn part2(values: &[String]) {
    let total_values = values.len();

    let oxygen_generator_id = find_index(values, (0..total_values).collect(), 0, true).unwrap();
    let co2_scrubber_id = find_index(values, (0..total_values).collect(), 0, false).unwrap();

    let oxygen_generator = &values[oxygen_generator_id];
    let co2_scrubber = &values[co2_scrubber_id];
    println!(
        "oxygen generator id: {}, value: {}",
        oxygen_generator_id, oxygen_generator
    );
    println!(
        "co2 scrubber id: {}, value: {}",
        co2_scrubber_id, co2_scrubber
    );

    println!(
        "part 2 result: {}",
        parse_binary(oxygen_generator) * parse_binary(co2_scrubber)
    );
}

fn find_index(
    values: &[String],
    indices: Vec<usize>,
    depth: usize,
    is_most_common: bool,
) -> Option<usize> {
    let total_values = indices.len();
    if total_values < 1 {
        return None;
    };
    if total_values == 1 {
        return Some(indices[0]);
    }

    let width = values[0].len();
    if depth >= width {
        return None;
    };

    let zeros = indices
        .iter()
        .filter(|i| values[**i].chars().nth(depth) == Some('0'))
        .map(|x| *x)
        .collect::<Vec<_>>();
    let ones = indices
        .iter()
        .filter(|i| values[**i].chars().nth(depth) == Some('1'))
        .map(|x| *x)
        .collect::<Vec<_>>();

    match (ones.len() >= zeros.len(), is_most_common) {
        (true, true) => find_index(values, ones, depth + 1, is_most_common),
        (true, false) => find_index(values, zeros, depth + 1, is_most_common),
        (false, true) => find_index(values, zeros, depth + 1, is_most_common),
        (false, false) => find_index(values, ones, depth + 1, is_most_common),
    }
}

fn parse_binary(text: &str) -> usize {
    usize::from_str_radix(text, 2).unwrap()
}
