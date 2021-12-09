use crate::util::{self, BResult, Boom};
use std::convert::TryInto;
use std::str::FromStr;
use std::collections::HashMap;

/******************

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

******************/

#[derive(Debug)]
struct DisplayNote {
    pub input: [String; 10],
    pub output: [String; 4],
}

impl FromStr for DisplayNote {
    type Err = Boom;

    fn from_str(s: &str) -> Result<DisplayNote, Boom> {
        let parts = s.split(" | ").collect::<Vec<_>>();
        let input: [String; 10] = sized_string(parts[0]);
        let output: [String; 4] = sized_string(parts[1]);

        Ok(DisplayNote {
            input: input,
            output: output,
        })
    }
}

type Input = [DisplayNote];

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day8.txt", "input/day8-test.txt")?;
    let input = util::parse_lines::<DisplayNote>(&input_string)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(notes: &Input) {
    fn decode_digit(signals: &String) -> Option<i32> {
        match signals.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    let mut decodable_count = 0;
    for n in notes {
        let hits_count = n
            .output
            .iter()
            .map(decode_digit)
            .filter(|d| d.is_some())
            .count();
        decodable_count += hits_count;
    }

    println!("part 1 solution: {}", decodable_count);
}

mod display_segment {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;
    pub const D: usize = 3;
    pub const E: usize = 4;
    pub const F: usize = 5;
    pub const G: usize = 6;
    pub const COUNT: usize = 7;
}

fn part2(notes: &Input) {
    use crate::seq_ops::IterOps;
    use display_segment::{A, B, C, COUNT, D, E, F, G};

    fn same(lhs: &str, rhs: &str) -> bool {
        let mut lhv: Vec<char> = lhs.chars().collect();
        let mut rhv: Vec<char> = rhs.chars().collect();
        lhv.sort();
        rhv.sort();
        lhv == rhv
    }

    fn find_by_length(digits: &[String], length: usize) -> Vec<usize> {
        digits
            .iter()
            .enumerate()
            .filter(|(_, x)| x.len() == length)
            .map(|(i, _)| i)
            .collect()
    }

    fn one_by_length(digits: &[String], length: usize) -> usize {
        find_by_length(digits, length)[0]
    }

    fn process_note(note: &DisplayNote) -> HashMap<char, char> {
        let signals = &note.input;
        let mut digit_to_signal = [0usize; 10];
        let mut sig_to_dis = [' '; COUNT];

        digit_to_signal[1] = one_by_length(signals, 2);
        digit_to_signal[4] = one_by_length(signals, 4);
        digit_to_signal[7] = one_by_length(signals, 3);
        digit_to_signal[8] = one_by_length(signals, 7);

        //println!("unique digits: {:?}", digit_to_signal);

        let mut sixes = find_by_length(signals, 6); // 0, 6, 9
        let mut six_to_remove = 0;
        for &i in sixes.iter() {
            let current = &signals[i];
            let intersection = current
                .chars()
                .intersect(signals[digit_to_signal[1]].chars());
            if intersection.len() == 1 {
                digit_to_signal[6] = i;
                let f = intersection[0];
                sig_to_dis[F] = f;
                sig_to_dis[C] = signals[digit_to_signal[1]]
                    .chars()
                    .find(|c| *c != f)
                    .unwrap();
                break;
            }
            six_to_remove += 1;
        }

        sixes.remove(six_to_remove); // 0, 9
        six_to_remove = 0;
        for &i in sixes.iter() {
            let current = &signals[i];
            let intersection = current
                .chars()
                .intersect(signals[digit_to_signal[4]].chars());
            if intersection.len() == 4 {
                digit_to_signal[9] = i;
                break;
            }
            six_to_remove += 1;
        }

        sixes.remove(six_to_remove); // 0
        digit_to_signal[0] = sixes[0];

        // --- now we know 0, 1, 4, 6, 7, 8, 9 ---
        // --- let's work on letter mappings   ---
        let a = signals[digit_to_signal[7]]
            .chars()
            .difference(signals[digit_to_signal[4]].chars());
        sig_to_dis[A] = a[0];

        let e = signals[digit_to_signal[8]]
            .chars()
            .difference(signals[digit_to_signal[9]].chars());
        sig_to_dis[E] = e[0];

        let d = signals[digit_to_signal[8]]
            .chars()
            .difference(signals[digit_to_signal[0]].chars());
        sig_to_dis[D] = d[0];

        let b = signals[digit_to_signal[4]].chars().difference(
            vec![sig_to_dis[C], sig_to_dis[D], sig_to_dis[F]]
                .iter()
                .collect::<String>()
                .chars(),
        );
        sig_to_dis[B] = b[0];

        let g = signals[digit_to_signal[8]]
            .chars()
            .difference(
                vec![sig_to_dis[A], sig_to_dis[B], sig_to_dis[C], sig_to_dis[D], sig_to_dis[E], sig_to_dis[F]]
                    .iter()
                    .collect::<String>()
                    .chars(),
            );
        sig_to_dis[G] = g[0];

        //println!("story so far: {:?}", (A..=G).zip(sig_to_dis.iter()).collect::<Vec<_>>());
        sig_to_dis.into_iter().zip('a'..='g').collect::<HashMap<char, char>>()
    }

    fn translate(word: &str, dictionary: &HashMap<char, char>) -> String {
        word.chars().map(|c| dictionary[&c]).collect()
    }

    fn display_to_digit(word: &str) -> Option<i32> {
        let mut letters = word.chars().collect::<Vec<_>>();
        letters.sort();
        let sorted_word: String = letters.into_iter().collect::<String>();
        match sorted_word.as_str() {
            "abcefg" => Some(0),
            "cf" => Some(1),
            "acdeg" => Some(2),
            "acdfg" => Some(3),
            "bcdf" => Some(4),
            "abdfg" => Some(5),
            "abdefg" => Some(6),
            "acf" => Some(7),
            "abcdefg" => Some(8),
            "abcdfg" => Some(9),
            _ => None
        }
    }

    let mut sum = 0;
    for note in notes.iter() {
        let signal_to_display = process_note(note);
        dbg!(&signal_to_display);

        let mut num = 0;
        for o in &note.output {
            let translated = translate(&o, &signal_to_display);
            let digit = display_to_digit(&translated).unwrap();
            num = num * 10 + digit;
        }
        sum += num;
    }

    println!("part 2 solution: {}", sum);
}

fn sized_string<const N: usize>(s: &str) -> [String; N] {
    s.trim()
        .split(" ")
        .map(String::from)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
