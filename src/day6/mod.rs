const DAY_NUM: &str = "6";

fn solve(input: &str) -> [String; 2] {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = lines[0].split_once(':').unwrap().1
        .split(' ').filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let dists: Vec<u32> = lines[1].split_once(':').unwrap().1
        .split(' ').filter_map(|s| s.parse::<u32>().ok())
        .collect();
    
    [
        "todo".to_owned(), 
        "todo".to_owned()
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