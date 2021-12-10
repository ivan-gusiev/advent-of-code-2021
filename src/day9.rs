use crate::util::{self, BResult, Boom};
use image::{ImageBuffer, Rgb};
use std::str::FromStr;

type Input = Table;

type Point = (usize, usize);

struct Table {
    rows: Vec<Row>,
}

impl Table {
    fn from_rows(rows: Vec<Row>) -> Table {
        Table { rows: rows }
    }

    fn width(&self) -> usize {
        self.rows[0].heights.len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.rows[y].heights[x]
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<Point> {
        let mut results: Vec<(usize, usize)> = Vec::with_capacity(4);

        if y > 0 {
            results.push((x, y - 1));
        }
        if x > 0 {
            results.push((x - 1, y));
        }
        if y < self.height() - 1 {
            results.push((x, y + 1))
        }
        if x < self.width() - 1 {
            results.push((x + 1, y))
        }

        results
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let mut result = true;
        let current_height = self.get(x, y);
        for (nx, ny) in self.neighbours(x, y) {
            if self.get(nx, ny) <= current_height {
                result = false;
                break;
            }
        }
        result
    }

    fn get_low_points(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::with_capacity(self.height()); // seems like a reasonal estimate
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.is_low_point(x, y) {
                    result.push((x, y))
                }
            }
        }
        result
    }
}

struct Row {
    heights: Vec<i32>,
}

impl FromStr for Row {
    type Err = Boom;

    fn from_str(s: &str) -> Result<Row, Boom> {
        s.chars()
            .map(|c| c.to_string().parse::<i32>().map_err(Boom::from_display))
            .collect::<Result<Vec<i32>, Boom>>()
            .map(|x| Row { heights: x })
    }
}

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day9.txt", "input/day9-test.txt")?;
    let input = Table::from_rows(util::parse_lines::<Row>(&input_string)?);

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(table: &Input) {
    let sum: i32 = table
        .get_low_points()
        .iter()
        .map(|(x, y)| table.get(*x, *y) + 1)
        .sum();

    println!("part 1 solution: {}", sum);
}

fn part2(table: &Input) {
    let low_points = table.get_low_points();
    let mut basins = low_points
        .iter()
        .map(|p| {print!("."); generate_basin(table, *p)})
        .collect::<Vec<_>>();
    println!("basins found");

    paint_caves(table, &basins).unwrap();

    basins.sort_by_key(|b| -(b.len() as i32));
    let result: usize = basins.iter().take(3).map(|x| x.len()).product();

    println!("part 2 solution: {}", result);
}

fn paint_caves(map: &Table, basins: &Vec<Vec<Point>>) -> BResult<()> {
    let all_basins = basins.iter().flatten().collect::<Vec<_>>();
    let img = ImageBuffer::from_fn(map.width() as u32, map.height() as u32, |x, y| {
        let (x, y) = (x as usize, y as usize);
        let luma = (map.get(x, y) * 255 / 10) as u8;
        if all_basins.contains(&&(x, y)) {
            Rgb([luma, 0, 0])
        } else {
            Rgb([luma, luma, luma])
        }
    });

    Ok(img.save("output/day9.png")?)
}

fn generate_basin(table: &Table, starting_point: Point) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(10);
    let mut queue: Vec<Point> = Vec::with_capacity(10);

    queue.push(starting_point);

    while queue.len() > 0 {
        let current @ (curx, cury) = queue.pop().unwrap();
        result.push(current);

        let cur_height = table.get(curx, cury);
        let neighbours = table.neighbours(curx, cury);

        let basin_points = neighbours
            .iter()
            .filter(|(nx, ny)| (cur_height..9).contains(&table.get(*nx, *ny)))
            .filter(|p| !queue.contains(p) && !result.contains(p))
            .collect::<Vec<_>>();

        basin_points.iter().for_each(|p| queue.push(**p));
    }

    result
}
