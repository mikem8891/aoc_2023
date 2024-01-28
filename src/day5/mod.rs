const DAY_NUM: &str = "5";

use std::collections::HashMap;

struct SeedMap(Vec<[u64; 3]>);

impl SeedMap {
    fn get(&self, input: u64) -> u64 {
        let in_range = |sm| 0 <= input && input < sm[2];
        let result = self.map(|s| input - s[1])
            .filter(|r| 0 <= input && input < r[2])
            .map(|d| input + d[0])
            .next();
        if let Some(output) = result {
            output
        } else {
            input
        }
    }
}

fn seed_map(text: &str) -> (String, SeedMap) {
    let (name, nums) = text.split_once(":\n");
    let mut maps: Vec<[u64; 3]> = vec![];
    for map in nums.lines() {
        let mut map = map.split(' ').filter_map(|s| s.parse::<u64>().ok());
        let get_next = |&mut iter| iter.next().unwrap();
        let map = [get_next(&map), get_next(&map), get_next(&map)];
        maps.push(map);
    }
    maps.sort_by_key(|&m| m[1]);
    (name.to_owned(), SeedMap(maps))
}

fn solve(input: &str) -> [String; 2] {
    let (seeds, almanac) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let seed_maps: HashMap<_, _> = almanac.split("/n/n")
        .map(seed_map)
        .collect();
    const MAP_ORDER: [&str; ] = [
        ""
    ];
    
    todo!();
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