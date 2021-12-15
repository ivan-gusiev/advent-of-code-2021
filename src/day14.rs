use crate::util::{self, BResult, Boom};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    template: String,
    rules: Vec<Rule>,
}

impl FromStr for Input {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Input, Boom> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let template = parts[0].trim();
        let rules = util::parse_lines::<Rule>(parts[1])?;
        Ok(Input {
            template: template.to_string(),
            rules: rules,
        })
    }
}

#[derive(Debug)]
struct Rule(char, char, char);

impl FromStr for Rule {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Rule, Boom> {
        let chars: Vec<char> = s.chars().collect();
        Ok(Rule(chars[0], chars[1], chars[6]))
    }
}

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day14.txt", "input/day14-test.txt")?;
    let input = input_string.parse::<Input>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn update_polymer(polymer: &mut String, rule_map: &HashMap<String, String>) {
    let mut i = 0;
    while i < polymer.len()-1 {
        let range = i..i+2;
        let s = &polymer[range.clone()];

        match rule_map.get(s) {
            Some(output) => {
                polymer.replace_range(range, output);
                i += 1;
            }
            None => ()
        }
        i += 1;
    }
}

fn calculate_score(polymer: &String) -> usize {
    let chars = util::count_items(polymer.chars());
    let (_, max) = chars.iter().max_by_key(|(_, v)| *v).unwrap();
    let (_, min) = chars.iter().min_by_key(|(_, v)| *v).unwrap();
    *max - *min
}

fn build_rule_map(input: &Input) -> HashMap<String, String> {
    let mut rule_map: HashMap<String, String> = HashMap::with_capacity(input.rules.len());
    for Rule(i1, i2, o) in input.rules.iter() {
        let input_str = String::from_iter([i1, i2]);
        let output_str = String::from_iter([i1, o, i2]);
        rule_map.insert(input_str, output_str);
    }
    rule_map
}

fn part1(input: &Input) {
    let mut polymer = input.template.to_string();
    let rule_map = build_rule_map(input);

    for _ in 0..10 {
        update_polymer(&mut polymer, &rule_map);
    }

    println!("part 1 solution: {}", calculate_score(&polymer));
}

fn part2(input: &Input) {
    let mut polymer = input.template.to_string();
    let rule_map = build_rule_map(input);

    for i in 0..40 {
        update_polymer(&mut polymer, &rule_map);
        println!("step {} polymer size {}", i, polymer.len());
    }

    println!("part 2 solution: {}", calculate_score(&polymer));
}
