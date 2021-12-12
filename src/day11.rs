use crate::util::{self, BResult, Boom};
use gif::{Encoder, Frame};
use std::str::FromStr;

type Input = Table;

type Point = (usize, usize);

#[derive(Clone, Debug)]
struct Table {
    rows: Vec<Row>,
}

impl Table {
    fn from_rows(rows: Vec<Row>) -> Table {
        Table { rows: rows }
    }

    fn width(&self) -> usize {
        self.rows[0].values.len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.rows[y].values[x]
    }

    fn set(&mut self, x: usize, y: usize, val: i32) {
        self.rows[y].values[x] = val
    }

    fn paint<W: std::io::Write>(&self, encoder: &mut Encoder<W>) -> BResult<()> {
        let pixels = self
            .iter()
            .map(|x| std::cmp::min(*x, 9) as u8)
            .collect::<Vec<_>>();
        let frame =
            Frame::from_indexed_pixels(self.width() as u16, self.height() as u16, &pixels, None);
        encoder.write_frame(&frame)?;
        Ok(())
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<Point> {
        let mut results: Vec<(usize, usize)> = Vec::with_capacity(4);
        let (w, h) = (self.width(), self.height());

        for nx in (if x == 0 { x } else { x - 1 })..=x + 1 {
            for ny in (if y == 0 { y } else { y - 1 })..=y + 1 {
                if nx >= w || ny >= h {
                    continue;
                }
                if (x, y) == (nx, ny) {
                    continue;
                }
                results.push((nx, ny))
            }
        }

        results
    }

    fn iter(&self) -> impl Iterator<Item = &i32> {
        self.rows.iter().map(|x| x.values.iter()).flatten()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut i32> {
        self.rows.iter_mut().map(|x| x.values.iter_mut()).flatten()
    }

    fn find_flasher(&self, processed: &Table) -> Option<Point> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y);
                if val > 9 && processed.get(x, y) == 0 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn deenergize(&mut self, processed: &Table) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if processed.get(x, y) > 0 {
                    self.set(x, y, 0)
                }
            }
        }
    }

    fn update(&mut self) -> i32 {
        let mut flashes = self.clone();
        flashes.iter_mut().for_each(|v| *v = 0);

        self.iter_mut().for_each(|v| *v += 1);

        while let Some((x, y)) = self.find_flasher(&flashes) {
            flashes.set(x, y, 1);
            for (nx, ny) in self.neighbours(x, y) {
                self.set(nx, ny, self.get(nx, ny) + 1)
            }
        }

        self.deenergize(&flashes);
        flashes.iter().map(|x| if *x > 0 { 1 } else { 0 }).sum()
    }
}

#[derive(Clone, Debug)]
struct Row {
    values: Vec<i32>,
}

impl FromStr for Row {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Row, Boom> {
        s.chars()
            .map(|c| c.to_string().parse::<i32>().map_err(Boom::from_display))
            .collect::<Result<Vec<i32>, Boom>>()
            .map(|x| Row { values: x })
    }
}

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day11.txt", "input/day11-test.txt")?;
    let input = Table::from_rows(util::parse_lines::<Row>(&input_string)?);

    let mut palette = vec![255u8, 255u8, 0u8];
    for i in 1..=9 {
        let percentage = (i as f64) / 9f64 * 255f64;
        palette.push(0u8);
        palette.push(0u8);
        palette.push(percentage as u8);
    }

    let mut image = std::fs::File::create("output/day11.gif")?;
    let mut encoder = Encoder::new(
        &mut image,
        input.width() as u16,
        input.height() as u16,
        &palette,
    )?;

    part1(&input);
    part2(&input, &mut encoder);

    Ok(())
}

fn part1(table: &Input) {
    let mut field = table.clone();
    let mut sum_flashes = 0i64;
    for _ in 0..100 {
        let flashes_i = field.update() as i64;
        sum_flashes += flashes_i;
    }

    println!("part 1 solution: {}", sum_flashes);
}

fn part2<W: std::io::Write>(table: &Input, encoder: &mut Encoder<W>) {
    let mut field = table.clone();

    let mut step: i32 = 0;
    for i in 0.. {
        let flashes_i = field.update() as i64;
        field.paint(encoder).unwrap();
        if flashes_i == 100 {
            step = i;
            break;
        }
    }

    println!("part 2 solution: {}", step + 1);
}
