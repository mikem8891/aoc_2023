const DAY_NUM: &str = "5";

struct SeedMap {
    name: String,
    maps: Vec<[u64; 3]>
}

impl SeedMap {
    fn new(text: &str) -> SeedMap {
        let (name, nums) = text.split_once(":\n");
        let mut maps: Vec<[u64; 3]> = vec![];
        for map in nums.lines() {
            let mut map = map.split(' ').filter_map(|s| s.parse::<u64>().ok());
            let get_next = |&mut iter| iter.next().unwrap();
            let map = [get_next(&map), get_next(&map), get_next(&map)];
            maps.push(map);
        }
        SeedMap {name, maps}
    }
}

fn solve(input: &str) -> [String; 2] {
    let (seeds, almanac) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let seed_maps: Vec<_> = almanac.split("/n/n").map(SeedMap::new).collect();
    
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