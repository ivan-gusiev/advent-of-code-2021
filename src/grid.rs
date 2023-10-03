use crate::util::{self, Boom};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct Row<T> {
    values: Vec<T>,
}

impl<T> FromStr for Row<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = Boom;

    fn from_str(s: &str) -> Result<Row<T>, Boom> {
        s.chars()
            .map(|c| c.to_string().parse::<T>().map_err(Boom::from_display))
            .collect::<Result<Vec<_>, Boom>>()
            .map(|x| Row { values: x })
    }
}

impl<T> Row<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: Vec<Row<T>>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Row<T>>) -> Grid<T> {
        Grid { rows: rows }
    }

    pub fn width(&self) -> usize {
        self.rows.get(0).map(|v| v.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, Point(x, y): Point) -> &T {
        &self.rows[y].values[x]
    }

    pub fn set(&mut self, Point(x, y): Point, val: T) {
        self.rows[y].values[x] = val
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (w, h) = (self.width(), self.height());

        self.rows
            .iter()
            .take(h)
            .map(move |x| x.values.iter().take(w))
            .flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let (w, h) = (self.width(), self.height());

        self.rows
            .iter_mut()
            .take(h)
            .map(move |x| x.values.iter_mut().take(w))
            .flatten()
    }
}

impl<T> Grid<T>
where
    T: std::fmt::Display
{
    pub fn print(&self) {
        for r in self.rows.iter().take(self.height()) {
            for v in r.values.iter().take(self.width()) {
                print!("{}", v)
            }
            println!();
        }
    }
}

impl<T> Grid<T>
where
    T: Copy + Default
{
    pub fn from_size(width: usize, height: usize) -> Grid<T> {
        let row = Row {
            values: vec![T::default(); width],
        };
        let rows = vec![row; height];
        Grid::from_rows(rows)
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn blit_from(&mut self, source: &Grid<T>, Point(atx, aty): Point) {
        for y in 0..source.height() {
            for x in 0..source.width() {
                let from = Point(x, y);
                let to = Point(x + atx, y + aty);
                self.set(to, *source.get(from));
            }
        }
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = Boom;

    fn from_str(text: &str) -> Result<Grid<T>, Boom> {
        Ok(Grid::from_rows(util::parse_lines::<Row<T>>(text)?))
    }
}
