#[allow(dead_code)]
const DAY_NUM: &str = "5";

use std::{
    collections::HashMap, 
    ops::Range,
    cmp::{min, max}
};

#[derive(Clone)]
struct SeedMap(Box<[SeedSubMap]>);

#[derive(Clone)]
#[allow(dead_code)]
struct SeedSubMap {
    des: u64,
    src: u64,
    rng: u64
}

impl SeedMap {
    #[allow(dead_code)]
fn get(&self, input: u64) -> u64 {
        self.0.iter()
            .filter(|r| input < r.src + r.rng)
            .take_while(|r| r.src <= input)
            .map(|d| input + d.des - d.src)
            .next().unwrap_or(input)
    }

    #[allow(dead_code)]
fn get_range<'a>(&'a self, input: Range<u64>) -> impl Iterator<Item = Range<u64>> + 'a {
        let new_range = move |ssm: &SeedSubMap| {
            let src = max(input.start, ssm.src)..min(input.end, ssm.src + ssm.rng);
            if src.is_empty() {
                None
            } else {
                Some((src.start + ssm.des - ssm.src)..(src.end + ssm.des - ssm.src))
            }
        };
        self.0.iter()
            .take_while(move |r| r.src <= input.end)
            .filter_map(new_range)
    }
}

#[allow(dead_code)]
fn seed_map(text: &Vec<&str>) -> (String, SeedMap) {
    let mut lines = text.iter();
    let (name, _) = lines.next().unwrap().split_once(":").unwrap();
    let mut maps: Vec<_> = vec![];
    for map in lines {
        let mut map = map.split(' ').filter_map(|s| s.parse::<u64>().ok());
        let map = SeedSubMap {
            des: map.next().unwrap(),
            src: map.next().unwrap(),
            rng: map.next().unwrap()
        }        ;
        maps.push(map);
    }
    maps.sort_by_key(|m| m.src);
    (name.to_owned(), SeedMap(maps.into_boxed_slice()))
}

#[allow(dead_code)]
fn seed_to_loc(ordered_seed_map: &[SeedMap], seed: u64) -> u64 {
    ordered_seed_map.iter()
        .fold(seed, |s, sm| sm.get(s))
}

#[allow(dead_code)]
fn seed_to_loc_range(ordered_seed_map: &[SeedMap], seed_range: Range<u64>) -> Vec<Range<u64>> {
    ordered_seed_map.into_iter()
        .fold(vec![seed_range], |s, sm| s.into_iter().map(|s| sm.get_range(s)).flatten().collect())
}

#[allow(dead_code)]
fn solve(input: &str) -> [String; 2] {
    let mut lines = input.lines().peekable();
    let seeds: Vec<_> = lines.by_ref()
        .take_while(|l| l.trim().len() > 0).collect();
    let almanac = {
        let mut blocks = vec![];
        while lines.peek().is_some() {
            let block = lines.by_ref()
                .take_while(|l| l.trim().len() > 0).collect();
            blocks.push(block);
        }
        blocks
    };
    let seeds: Vec<u64> = seeds.iter().map(|s| s.split(' ')).flatten()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut seed_iter = seeds.iter().peekable();
    let mut seed_ranges = vec![];
    while seed_iter.peek().is_some() {
        let seed_start = seed_iter.next().unwrap();
        let seed_end = seed_start + seed_iter.next().unwrap();
        seed_ranges.push(*seed_start..seed_end);
    }
    let seed_maps: HashMap<_, _> = almanac.iter()
        .map(seed_map)
        .collect();
    let map_seeds = |s: Vec<u64>, m: &&SeedMap| 
        s.iter().map(|s| m.get(*s)).collect::<Vec<_>>();
    const MAP_ORDER: [&str; 7] = [
        "seed-to-soil map",
        "soil-to-fertilizer map",
        "fertilizer-to-water map",
        "water-to-light map",
        "light-to-temperature map",
        "temperature-to-humidity map",
        "humidity-to-location map"
    ];
    let locations = MAP_ORDER.map(|n| &seed_maps[&(n.to_owned())])
        .iter().fold(seeds, map_seeds);
    let ordered_seed_map: Box<[SeedMap]> = MAP_ORDER.iter()
        .map(|mt| seed_maps[*mt].clone()).collect();
    let location_p2 = seed_ranges.iter()
        .map(|sr| sr.clone().into_iter()).flatten()
        .map(|s| seed_to_loc(ordered_seed_map.as_ref(), s))
        .min().unwrap();
//    let location_p3 = seed_ranges.into_iter()
//        .map(|sr| seed_to_loc_range(ordered_seed_map.as_ref(), sr).into_iter()).flatten()
//        .map(|r| r.start)
//        .min().unwrap();
    [
        locations.iter().min().unwrap().to_string(), 
        location_p2.to_string()
    ]
}

#[allow(dead_code)]
pub fn main() {
    println!("Day 5");
    let path = format!("./src/day{DAY_NUM}/input.txt");
    let input = std::fs::read_to_string(path).unwrap();
    let [part_1, part_2] = solve(&input);
    println!("part 1 is {part_1}");
    println!("part 2 is {part_2}");
}

#[cfg(test)]
mod test;