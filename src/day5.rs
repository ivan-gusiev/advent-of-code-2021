use crate::util::{parse_lines, read_file_maybe_test, BResult, Boom};
use image::{ImageBuffer, Luma};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct VentLine {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl VentLine {
    pub fn all_coords(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {
            let (y1, y2) = if self.y1 > self.y2 {
                (self.y2, self.y1)
            } else {
                (self.y1, self.y2)
            };
            return (y1..=y2).map(|y| (self.x1, y)).collect::<Vec<_>>();
        }

        if self.y1 == self.y2 {
            let (x1, x2) = if self.x1 > self.x2 {
                (self.x2, self.x1)
            } else {
                (self.x1, self.x2)
            };
            return (x1..=x2).map(|x| (x, self.y1)).collect::<Vec<_>>();
        }

        if (self.y2 as i32 - self.y1 as i32).abs() == (self.x2 as i32 - self.x1 as i32).abs() {
            let ((x1, y1), (x2, y2)) = if self.x1 > self.x2 {
                ((self.x2, self.y2), (self.x1, self.y1))
            } else {
                ((self.x1, self.y1), (self.x2, self.y2))
            };
            let ystep: i32 = if y2 > y1 { 1 } else { -1 };
            return (x1..=x2)
                .map(|x| (x, (y1 as i32 + ystep * (x as i32 - x1 as i32)) as usize))
                .collect::<Vec<_>>();
        }

        vec![]
    }
}

lazy_static! {
    static ref VENTLINE_RE: Regex =
        Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();
}

impl FromStr for VentLine {
    type Err = Boom;

    fn from_str(s: &str) -> Result<VentLine, Boom> {
        fn as_int(s: &str) -> Result<usize, Boom> {
            Ok(s.parse::<usize>().map_err(Boom::from_display)?)
        }

        match VENTLINE_RE.captures_iter(s).next() {
            Some(m) => Ok(VentLine {
                x1: as_int(&m["x1"])?,
                y1: as_int(&m["y1"])?,
                x2: as_int(&m["x2"])?,
                y2: as_int(&m["y2"])?,
            }),
            None => Err(Boom {
                value: format!("Could not parse input {}", s),
            }),
        }
    }
}

struct Input {
    pub vent_lines: Vec<VentLine>,
}

pub fn run() -> BResult<()> {
    let input_string = read_file_maybe_test("input/day5.txt", "input/day5-test.txt")?;
    let input = Input {
        vent_lines: parse_lines(&input_string)?,
    };

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Input) {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.vent_lines.iter() {
        max_x = std::cmp::max(max_x, std::cmp::max(line.x1, line.x2));
        max_y = std::cmp::max(max_y, std::cmp::max(line.y1, line.y2));
    }

    let mut vent_map = vec![vec![0; max_x + 1]; max_y + 1];
    for line in input.vent_lines.iter() {
        for (x, y) in line.all_coords() {
            vent_map[y][x] = vent_map[y][x] + 1
        }
    }

    let max_depth = vent_map.iter().flatten().max().unwrap();
    dbg!(max_depth);

    let max_count = vent_map.iter().flatten().filter(|x| **x > 1).count();

    paint(&vent_map, *max_depth).unwrap();

    println!("part 1 solution: {}", max_count)
}

fn part2(_input: &Input) {
    println!("part 2 solution: {}", "oops")
}

fn paint(map: &Vec<Vec<i32>>, max_depth: i32) -> BResult<()> {
    let img = ImageBuffer::from_fn(
        map[0].len().try_into().unwrap(),
        map.len().try_into().unwrap(),
        |x, y| {
            Luma([(map[y as usize][x as usize] * 255 / max_depth) as u8])
            /*if x % 2 == 0 {
                Luma([0u8])
            } else {
                Luma([255u8])
            }>*/
        },
    );

    Ok(img.save("output/day5.png")?)
}
