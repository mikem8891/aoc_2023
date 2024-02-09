const DAY_NUM: &str = "9";

fn next_num(nums: &[u64]) -> u64 {
    
}

fn solve(input: &str) -> [String; 2] {
    let 
    let sum = input.lines()
        .map(|l| next_num(l.split(' ').filter_map(|n| n.parse::<u64>().ok()).collect::<Vec<_>>()))
        .sum().unwrap()
    
    [
        sum.to_string(),
        "todo".to_string()
    ]
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let path = format!("./src/day{DAY_NUM}/input.txt");
    let input = std::fs::read_to_string(path).unwrap();
    let [part_1, part_2] = solve(&input);
    println!("part 1 is {part_1}");
    println!("part 2 is {part_2}");
}

#[cfg(test)]
mod test;