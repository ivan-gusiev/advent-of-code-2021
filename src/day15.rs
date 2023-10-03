use crate::grid::{Grid, Point};
use crate::util::{self, BResult};
use std::collections::HashMap;

type Input = Grid<i32>;
type Path = (Vec<Point>, i32);
type Cache = HashMap<Point, Path>;

pub fn run() -> BResult<()> {
    let input_string = util::read_file_maybe_test("input/day15.txt", "input/day15-test.txt")?;
    let input = input_string.parse::<Input>()?;

    part1(&input);
    //part2(&input);

    Ok(())
}

fn left(Point(x, y): Point) -> Point {
    Point(x - 1, y)
}

fn right(Point(x, y): Point) -> Point {
    Point(x + 1, y)
}

fn up(Point(x, y): Point) -> Point {
    Point(x, y - 1)
}

fn down(Point(x, y): Point) -> Point {
    Point(x, y + 1)
}

fn append((path, cost): &Path, pt: Point, c: i32) -> Path {
    let mut result = path.clone();
    result.push(pt);
    (result, cost + c)
}

fn optimal_cost(from: Point, to: Point, map: &Input, so_far: &Path, mut cache: &mut Cache) -> Option<Path> {
    if so_far.0.iter().find(|p| **p == from).is_some() {
        return None;
    }
    
    if let Some((vec, cost)) = cache.get(&from) {
        return Some((vec.clone(), *cost));
    }

    let cur = *map.get(from);
    if from == to {
        return Some((vec![from], cur));
    }

    let Point(curx, cury) = from;
    let Point(tox, toy) = to;

    let new_so_far = append(so_far, from, cur);
    let mut options: Vec<Path> = vec![];
    if curx > 0 {
        if let Some(x) = optimal_cost(left(from), to, map, &new_so_far, &mut cache) {
            options.push(append(&x, from, cur))
        }
    }
    if curx < tox {
        if let Some(x) = optimal_cost(right(from), to, map, &new_so_far, &mut cache) {
            options.push(append(&x, from, cur))
        }
    }
    if cury > 0 {
        if let Some(x) = optimal_cost(up(from), to, map, &new_so_far, &mut cache) {
            options.push(append(&x, from, cur))
        }
    }
    if cury < toy {
        if let Some(x) = optimal_cost(down(from), to, map, &new_so_far, &mut cache) {
            options.push(append(&x, from, cur))
        }
    }

    if let Some((vec, cst)) = options.into_iter().min_by_key(|(_, c)| *c) {
        if let Some((_, ccst)) = cache.get(&from) {
            println!("asdfa");
            if cst < *ccst {
                cache.insert(from, (vec.clone(), cst));    
            }
        } else {
            cache.insert(from, (vec.clone(), cst));
        }
        Some((vec, cst))
    } else {
        None
    }
}

fn expand_input(input: &Input) -> Input {
    fn wrap(x: i32) -> i32 {
        if x > 9 { x - 9 } else { x }
    }

    let mut output = Input::from_size(input.width() * 5, input.height() * 5);
    for y in 0..5 {
        for x in 0..5 {
            let target = Point(x * input.width(), y * input.height());
            let riskiness = (x + y) as i32;
            let mut shard = input.clone();
            shard.iter_mut().for_each(|x| *x = wrap(*x + riskiness));

            output.blit_from(&shard, target)
        }
    }
    output
}

fn part1(input: &Input) {
    let mut answers = Cache::with_capacity(input.width() * input.height());
    let from = Point(0, 0);
    let to = Point(input.width() - 1, input.height() - 1);

    let (path, cost) = optimal_cost(from, to, &input, &(vec![], 0), &mut answers).unwrap();

    let mut canvas = Input::from_size(input.width(), input.height());
    let mut scan = 0;
    for &p in path.iter().rev() {
        scan += input.get(p);
        canvas.set(p, 1);
        println!("{:?} ({})", p, scan);
    }

    input.print();
    canvas.print();
    
    println!("part 1 solution: {}", cost - input.get(from));
}

/*fn part2(input: &Input) {
    let input = expand_input(input);
    let mut answers = Cache::with_capacity(input.width() * input.height());
    let from = Point(0, 0);
    let to = Point(input.width() - 1, input.height() - 1);

    let result = optimal_cost(from, to, &input, &mut answers, &mut vec![]).unwrap();
    println!("part 2 solution: {} ({})", result - input.get(from), result);
}*/
