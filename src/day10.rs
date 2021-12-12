use crate::util::{self, BResult};

type Input = [String];

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day10.txt", "input/day10-test.txt")?;
    let input = util::parse_lines::<String>(&input_string)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn reflect(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        c => c,
    }
}

fn syntax_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn auto_score(c: char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn get_syntax_score(line: &str) -> i32 {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match stack.pop() {
                None => return syntax_score(c),
                Some(t) => {
                    if t != reflect(c) {
                        return syntax_score(c);
                    }
                }
            },
            _ => (),
        }
    }
    0
}

fn part1(lines: &Input) {
    let mut total_score: i32 = 0;
    for line in lines {
        total_score += get_syntax_score(line)
    }

    println!("part 1 solution: {}", total_score);
}

fn part2(lines: &Input) {
    fn repair_line(line: &str) -> String {
        let mut stack = Vec::<char>::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => match stack.pop() {
                    None => panic!("error"),
                    Some(t) => {
                        if t != reflect(c) {
                            panic!("error")
                        }
                    }
                },
                _ => (),
            }
        }
        for c in stack.iter_mut() {
            *c = reflect(*c)
        }
        stack.reverse();
        stack.iter().collect()
    }

    fn get_auto_score(line: &str) -> i64 {
        let missing = repair_line(line);
        let mut score = 0i64;
        for c in missing.chars() {
            score *= 5;
            score += auto_score(c) as i64;
        }
        score
    }

    let mut result = lines
        .iter()
        .filter(|s| get_syntax_score(&s) == 0)
        .map(|s| get_auto_score(&s))
        .collect::<Vec<_>>();

    println!("part 2 solution: {}", util::median64(&mut result));
}
