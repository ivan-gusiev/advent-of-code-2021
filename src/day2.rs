use crate::util::{parse_lines, read_file, BResult, Boom};
use std::str::FromStr;

struct Instruction {
    pub command: String,
    pub value: i32,
}

impl FromStr for Instruction {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Instruction, Boom> {
        let spl = s.split_ascii_whitespace().collect::<Vec<_>>();
        let val = spl[1].parse::<i32>().map_err(Boom::from_display)?;
        Ok(Instruction {
            command: spl[0].to_string(),
            value: val,
        })
    }
}

pub fn run() -> BResult<()> {
    let input = read_file("input/day2.txt")?;
    let instructions: Vec<Instruction> = parse_lines(&input)?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}

fn part1(instructions: &[Instruction]) {
    let mut depth = 0;
    let mut horizontal = 0;

    for instruction in instructions {
        match instruction.command.as_str() {
            "forward" => horizontal += instruction.value,
            "down" => depth += instruction.value,
            "up" => depth -= instruction.value,
            _ => println!("{}", "unknown command"),
        }
    }

    println!("part 1 result: {}", depth * horizontal);
}

fn part2(instructions: &[Instruction]) {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for instruction in instructions {
        match instruction.command.as_str() {
            "forward" => {
                horizontal += instruction.value;
                depth += aim * instruction.value;
            }
            "down" => aim += instruction.value,
            "up" => aim -= instruction.value,
            _ => println!("{}", "unknown command"),
        }
    }

    println!("part 2 result: {}", depth * horizontal);
}
