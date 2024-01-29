const DAY_NUM: &str = "5";

use std::collections::HashMap;

struct SeedMap(Vec<[u64; 3]>);

impl SeedMap {
    fn get(&self, input: u64) -> u64 {
        self.0.iter().map(|s| (input - s[1], *s))
            .filter(|(input, r)| 0 <= *input && input < &r[2])
            .map(|(input, d)| input + d[0])
            .next().unwrap_or(input)
    }
}

fn seed_map(text: &Vec<&str>) -> (String, SeedMap) {
    let mut lines = text.iter();
    let (name, _) = lines.next().unwrap().split_once(":").unwrap();
    let mut maps: Vec<[u64; 3]> = vec![];
    for map in lines {
        let mut map = map.split(' ').filter_map(|s| s.parse::<u64>().ok());
        let map = [
            map.next().unwrap(), 
            map.next().unwrap(), 
            map.next().unwrap()
        ];
        maps.push(map);
    }
    maps.sort_by_key(|&m| m[1]);
    (name.to_owned(), SeedMap(maps))
}

fn solve(input: &str) -> [String; 2] {
    let mut lines = input.lines().peekable();
    let seeds: Vec<_> = lines
        .take_while(|l| l.trim().len() > 0).collect();
    let almanac = {
        let mut blocks = vec![];
        while lines.peek().is_some() {
            let block = lines
                .take_while(|l| l.trim().len() > 0).collect();
            blocks.push(block);
        }
        blocks
    };
    let seeds: Vec<u64> = seeds.iter().map(|s| s.split(' ')).flatten()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
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
    [locations.iter().min().unwrap().to_string(), "todo".to_owned()]
}

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