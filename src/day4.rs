use crate::util::{parse_lines, read_file_maybe_test, split_parse, BResult, Boom};
use std::str::FromStr;

struct CardTemplate<T>
where
    T: Copy + Default,
{
    values: [[T; 5]; 5],
}

type Card = CardTemplate<i32>;

impl<T: Copy + Default> CardTemplate<T> {
    pub fn new() -> CardTemplate<T> {
        CardTemplate::<T> {
            values: [[<T as Default>::default(); 5]; 5],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        assert!(x < 5, "x >= 5");
        assert!(y < 5, "y >= 5");

        self.values[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        assert!(x < 5, "x >= 5");
        assert!(y < 5, "y >= 5");

        self.values[y][x] = val
    }
}

impl FromStr for Card {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Card, Boom> {
        let lines: Vec<String> = parse_lines(s).map_err(Boom::from_display)?;

        let nums = lines
            .iter()
            .map(|s| split_parse::<i32>(s, regex::Regex::new("\\s+")?))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Boom::from_display)?;

        let mut card = Card::new();
        for y in 0..5 {
            for x in 0..5 {
                card.set(x, y, nums[y][x])
            }
        }

        Ok(card)
    }
}

struct Input {
    pub drawn_numbers: Vec<i32>,
    pub cards: Vec<Card>,
}

impl FromStr for Input {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Input, Boom> {
        let lines: Vec<String> = s.split("\n\n").map(String::from).collect::<Vec<_>>();

        let drawn_numbers = lines[0]
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(Boom::from_display)?;

        let cards = lines
            .iter()
            .skip(1)
            .map(|s| Card::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Input {
            drawn_numbers: drawn_numbers,
            cards: cards,
        })
    }
}

type CardMarks = CardTemplate<bool>;

#[derive(Clone, Copy)]
enum WinCondition {
    Row(usize),
    Col(usize),
}

struct PlayingSession {
    pub card: Card,
    pub marks: CardMarks,
}

impl PlayingSession {
    pub fn from_card(card: Card) -> PlayingSession {
        PlayingSession {
            card: card,
            marks: CardMarks::new(),
        }
    }

    pub fn mark_number(&mut self, number: i32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.card.get(x, y) == number {
                    self.marks.set(x, y, true)
                }
            }
        }
    }

    pub fn check(&self) -> Option<(i32, WinCondition)> {
        let conditions = (0..5)
            .map(|i| [WinCondition::Row(i), WinCondition::Col(i)])
            .flatten()
            .collect::<Vec<_>>();

        for condition in conditions {
            let result = self.check_condition(condition);
            if result.is_some() {
                return result.map(|score| (score, condition));
            }
        }

        None
    }

    fn check_condition(&self, condition: WinCondition) -> Option<i32> {
        let mut marked_count = 0;
        let coords = match condition {
            WinCondition::Col(x) => (0..5).map(|y| (x, y)).collect::<Vec<_>>(),
            WinCondition::Row(y) => (0..5).map(|x| (x, y)).collect::<Vec<_>>(),
        };

        for (x, y) in coords {
            if self.marks.get(x, y) {
                marked_count += 1;
            }
        }

        if marked_count == 5 {
            let mut unmarked_sum = 0;
            for y in 0..5 {
                for x in 0..5 {
                    if !self.marks.get(x, y) {
                        unmarked_sum += self.card.get(x, y);
                    }
                }
            }
            Some(unmarked_sum)
        } else {
            None
        }
    }
}

pub fn run() -> BResult<()> {
    let input_string = read_file_maybe_test("input/day4.txt", "input/day4-test.txt")?;

    let input = Input::from_str(&input_string)?;
    part1(input);

    // recreating because we cannot be bothered to derive Clone
    let input = Input::from_str(&input_string)?;
    part2(input);

    Ok(())
}

fn part1(input: Input) {
    let mut sessions = input
        .cards
        .into_iter()
        .map(PlayingSession::from_card)
        .collect::<Vec<_>>();

    let mut result: i32 = -1;

    for num in input.drawn_numbers {
        sessions.iter_mut().for_each(|s| s.mark_number(num));

        let winner = sessions.iter().find(|c| c.check().is_some());
        match winner {
            None => (),
            Some(w) => {
                let (score, _condition) = w.check().unwrap();
                result = score * num;
                break;
            }
        }
    }

    println!("part 1 solution: {}", result)
}

fn part2(input: Input) {
    let mut sessions = input
        .cards
        .into_iter()
        .map(PlayingSession::from_card)
        .collect::<Vec<_>>();

    let mut result: i32 = -1;

    for num in input.drawn_numbers {
        sessions.iter_mut().for_each(|s| s.mark_number(num));

        let winners = sessions
            .iter()
            .enumerate()
            .filter(|(_, c)| c.check().is_some())
            .collect::<Vec<_>>();
        if winners.len() > 0 {
            for (_, w) in winners.iter() {
                let (score, _condition) = w.check().unwrap();
                result = score * num;
            }

            let mut to_remove = winners.into_iter().map(|(i, _)| i).collect::<Vec<_>>();
            to_remove.sort();
            to_remove.reverse();
            for i in to_remove {
                sessions.remove(i);
            }
        }
    }

    println!("part 2 solution: {}", result)
}
