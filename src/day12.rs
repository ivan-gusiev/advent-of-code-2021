use crate::util::{self, BResult, Boom};
use std::collections::HashMap;

fn is_big_cave(cave: &str) -> bool {
    cave.chars().nth(0).unwrap().is_uppercase()
}

type Input = MapGraph;

type Map = HashMap<String, Vec<String>>;

#[derive(Debug)]
struct MapGraph {
    data: Map,
}

impl MapGraph {
    fn from_edges(rows: Vec<EdgeRow>) -> MapGraph {
        let mut graph = MapGraph {
            data: Map::with_capacity(rows.len()),
        };

        for row in rows {
            graph.add_edge(&row.from, &row.to);
            graph.add_edge(&row.to, &row.from);
        }

        graph
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.data
            .entry(from.to_string())
            .or_insert_with(|| vec![])
            .push(to.to_string())
    }

    fn destinations(&self, from: &str) -> Vec<&str> {
        match self.data.get(from) {
            Some(ds) => ds.iter().map(|s| s.as_str()).collect(),
            None => vec![],
        }
    }
}

struct EdgeRow {
    from: String,
    to: String,
}

impl std::str::FromStr for EdgeRow {
    type Err = Boom;

    fn from_str(s: &str) -> Result<EdgeRow, Boom> {
        let parts: Vec<&str> = s.split("-").collect();

        Ok(EdgeRow {
            from: parts[0].trim().to_string(),
            to: parts[1].trim().to_string(),
        })
    }
}

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day12.txt", "input/day12-test.txt")?;
    let edges = util::parse_lines::<EdgeRow>(&input_string)?;

    let input = Input::from_edges(edges);

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Input) {
    println!("{:?}", input);

    fn get_all_paths(g: &MapGraph, from: &str, to: &str) -> Vec<Vec<String>> {
        fn get_all_paths_impl(
            g: &MapGraph,
            cur: &str,
            to: &str,
            path: Vec<String>,
        ) -> Vec<Vec<String>> {
            let mut result: Vec<Vec<String>> = vec![];

            for ds in g.destinations(cur) {
                if !is_big_cave(ds) && path.iter().find(|s| *s == ds).is_some() {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(ds.to_string());
                result.extend(get_all_paths_impl(g, ds, to, new_path));
            }

            if cur == to {
                result.push(path);
            }
            result
        }

        get_all_paths_impl(g, from, to, vec![from.to_string()])
    }

    let paths = get_all_paths(input, "start", "end");
    println!("part 1 solution: {}", paths.len());
}

#[derive(Clone, Debug)]
struct Path {
    segments: HashMap<String, i32>,
    road: Vec<String>,
    hvsct: bool,
}

impl Path {
    fn new() -> Path {
        Path {
            segments: HashMap::new(),
            road: vec![],
            hvsct: false,
        }
    }

    fn from_single(cave: &str) -> Path {
        let mut p = Path::new();
        p.visit(cave);
        p
    }

    fn visit(&mut self, cave: &str) {
        let cave = cave.to_string();
        let mut visit_count = 0;
        self.segments
            .entry(cave.clone())
            .and_modify(|cnt| {
                visit_count = *cnt + 1;
                *cnt = visit_count;
            })
            .or_insert(1);

        if visit_count > 1 && !is_big_cave(&cave) {
            self.hvsct = true;
        }

        self.road.push(cave);
    }

    fn has_visited_small_cave_twice(&self) -> bool {
        self.hvsct
    }

    fn count_visits(&self, cave: &str) -> i32 {
        *self.segments.get(cave).unwrap_or(&0)
    }

    fn is_starting_again(&self, cave: &str) -> bool {
        match &self.road.iter().nth(0) {
            Some(sp) => *sp == cave && self.road.len() > 1,
            None => false,
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?}",
            if self.has_visited_small_cave_twice() {
                "*"
            } else {
                " "
            },
            self.road
        )
    }
}

fn part2(input: &Input) {
    fn get_all_paths(g: &MapGraph, from: &str, to: &str) -> Vec<Path> {
        fn get_all_paths_impl(g: &MapGraph, cur: &str, to: &str, path: Path) -> Vec<Path> {
            if cur == to {
                // skip testing after reaching the end
                return vec![path];
            }

            if path.is_starting_again(cur) {
                // we're at start again
                return vec![];
            }

            let mut result: Vec<Path> = vec![];
            for ds in g.destinations(cur) {
                if !is_big_cave(ds) {
                    let visit_count = path.count_visits(ds);
                    if visit_count > 1 {
                        continue;
                    }
                    if visit_count > 0 && path.has_visited_small_cave_twice() {
                        continue;
                    }
                }

                let mut new_path = path.clone();
                new_path.visit(ds);
                result.extend(get_all_paths_impl(g, ds, to, new_path));
            }

            if cur == to {
                result.push(path);
            }
            result
        }

        get_all_paths_impl(g, from, to, Path::from_single(from))
    }

    let paths = get_all_paths(input, "start", "end");

    println!("part 2 solution: {}", paths.len());
}
