use crate::util::{read_file_maybe_test, split_parse, BResult, Boom};

const MAX_GEN: usize = 8;
const BIRTH_GEN: usize = 6;
const TOTAL_GENS: usize = 9;

#[derive(Debug)]
struct FishSpectrum {
    counts: [u64; TOTAL_GENS],
}

impl FishSpectrum {
    pub fn new() -> FishSpectrum {
        FishSpectrum {
            counts: [0; TOTAL_GENS],
        }
    }

    pub fn from_vec<T: Copy + TryInto<usize>>(input: &Vec<T>) -> Result<FishSpectrum, Boom>
    where
        T::Error: std::fmt::Debug,
    {
        let mut spectrum = FishSpectrum::new();

        for fish_value in input {
            let fish: usize = (*fish_value).try_into().unwrap();
            if fish > MAX_GEN {
                return Err(Boom::from_display(format!(
                    "generation cannot be larger than {} (was {})",
                    MAX_GEN, fish
                )));
            }
            spectrum.counts[fish] += 1;
        }

        Ok(spectrum)
    }

    pub fn update(&self) -> FishSpectrum {
        let births = self.counts[0];
        let mut new_spectrum = FishSpectrum::new();
        for i in 0..MAX_GEN {
            new_spectrum.counts[i] = self.counts[i + 1];
        }
        new_spectrum.counts[BIRTH_GEN] += births;
        new_spectrum.counts[MAX_GEN] += births;
        new_spectrum
    }

    pub fn show(&self) -> String {
        let gens = (0..MAX_GEN)
            .map(|x| std::iter::repeat(x).take(self.counts[x] as usize))
            .flatten()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        gens.join(",")
    }

    pub fn len(&self) -> u64 {
        self.counts.iter().sum()
    }
}

impl std::fmt::Display for FishSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}

pub fn run() -> BResult<()> {
    let input_string = read_file_maybe_test("input/day6.txt", "input/day6-test.txt")?;
    let input = split_parse::<i32>(&input_string, regex::Regex::new(",")?)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Vec<i32>) {
    let mut fish = FishSpectrum::from_vec(input).unwrap();

    println!("Initial state:   \t{}", fish);
    for _ in 1..=80 {
        fish = fish.update();
    }

    println!("part 1 solution: {}", fish.len());
}

fn part2(input: &Vec<i32>) {
    let mut fish = FishSpectrum::from_vec(input).unwrap();

    for _ in 1..=256 {
        fish = fish.update();
        //println!("Generation {} fish count {}", i, fish.len());
    }

    println!("part 2 solution: {}", fish.len());
}
