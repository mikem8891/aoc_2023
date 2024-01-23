const DAY_NUM: &str = "5";

fn solve(input: &str) -> [String; 2] {
    let (seeds, almanac) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
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