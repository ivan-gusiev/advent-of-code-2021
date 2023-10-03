use crate::util::{self, BResult, Boom};
use gif::{Encoder, Frame};
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Coordinate {
    X,
    Y,
}

impl FromStr for Coordinate {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Coordinate, Boom> {
        match s {
            "x" | "X" => Ok(Coordinate::X),
            "y" | "Y" => Ok(Coordinate::Y),
            _ => Err(Boom {
                value: format!("unknown coordinate {}", s),
            }),
        }
    }
}

#[derive(Clone, Debug)]
struct Point(usize, usize);
impl FromStr for Point {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Point, Boom> {
        let parts = s.split(",").collect::<Vec<_>>();
        let x = parts[0].trim().parse::<usize>()?;
        let y = parts[1].trim().parse::<usize>()?;
        Ok(Point(x, y))
    }
}

#[derive(Clone, Debug)]
struct Fold(Coordinate, usize);
impl FromStr for Fold {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Fold, Boom> {
        let s = s.replace("fold along ", "");
        let parts = s.split("=").collect::<Vec<_>>();
        let x = parts[0].parse::<Coordinate>()?;
        let y = parts[1].parse::<usize>()?;
        Ok(Fold(x, y))
    }
}

#[derive(Clone, Debug)]
struct Input {
    pub points: Vec<Point>,
    pub folds: Vec<Fold>,
}

impl FromStr for Input {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Input, Boom> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let points = util::parse_lines::<Point>(&parts[0])?;
        let folds = util::parse_lines::<Fold>(&parts[1])?;
        Ok(Input {
            points: points,
            folds: folds,
        })
    }
}

#[derive(Clone, Debug)]
struct Table {
    rows: Vec<Row>,
    effective_height: usize,
    effective_width: usize,
}

impl Table {
    fn from_points(points: &Vec<Point>) -> Table {
        let width = points.iter().map(|Point(x, _)| x).max().unwrap() + 1;
        let height = points.iter().map(|Point(_, y)| y).max().unwrap() + 1;

        let mut result = Table {
            rows: vec![
                Row {
                    values: vec![false; width]
                };
                height
            ],
            effective_height: height,
            effective_width: width,
        };
        for Point(x, y) in points {
            result.set(*x, *y, true);
        }
        result
    }

    fn width(&self) -> usize {
        self.effective_width
    }

    fn height(&self) -> usize {
        self.effective_height
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.rows[y].values[x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.rows[y].values[x] = val
    }

    fn iter(&self) -> impl Iterator<Item = &bool> {
        self.rows
            .iter()
            .take(self.effective_height)
            .map(|x| x.values.iter().take(self.effective_width))
            .flatten()
    }

    fn paint<W: std::io::Write>(&self, encoder: &mut Encoder<W>) -> BResult<()> {
        let pixels = self
            .iter()
            .map(|x| if *x { 1u8 } else { 0u8 })
            .collect::<Vec<_>>();
        let frame =
            Frame::from_indexed_pixels(self.width() as u16, self.height() as u16, &pixels, None);
        encoder.write_frame(&frame)?;
        Ok(())
    }

    fn print(&self) {
        for r in self.rows.iter().take(self.effective_height) {
            for v in r.values.iter().take(self.effective_width) {
                print!("{}", if *v { "#" } else { "." })
            }
            println!();
        }
    }

    fn fold(&mut self, f: &Fold) {
        match f {
            Fold(Coordinate::X, i) => self.fold_vertical(*i),
            Fold(Coordinate::Y, i) => self.fold_horizontal(*i),
        }
    }

    fn fold_horizontal(&mut self, row: usize) {
        for source_y in row + 1..self.height() {
            let target_y = row + row - source_y;
            for x in 0..self.width() {
                if self.get(x, source_y) {
                    self.set(x, target_y, true)
                }
            }
        }
        self.effective_height = row;
    }

    fn fold_vertical(&mut self, col: usize) {
        for source_x in col + 1..self.width() {
            let target_x = col + col - source_x;
            for y in 0..self.height() {
                if self.get(source_x, y) {
                    self.set(target_x, y, true)
                }
            }
        }
        self.effective_width = col;
    }
}

#[derive(Clone, Debug)]
struct Row {
    values: Vec<bool>,
}

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day13.txt", "input/day13-test.txt")?;
    let input = input_string.parse::<Input>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Input) {
    let mut table = Table::from_points(&input.points);
    table.fold(&input.folds[0]);

    let count_dots = table.iter().filter(|d| **d).count();

    println!("part 1 solution: {}", count_dots);
}

fn part2(input: &Input) {
    let mut table = Table::from_points(&input.points);

    let palette = vec![0u8, 0u8, 0u8, 255u8, 255u8, 255u8];

    for f in &input.folds {
        table.fold(f);
    }

    let mut image = std::fs::File::create("output/day13.gif").unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        table.width() as u16,
        table.height() as u16,
        &palette,
    )
    .unwrap();

    table.print();
    table.paint(&mut encoder).unwrap();
    println!("part 2 solution: {}", " LOOK UP ^^^ ");
}
