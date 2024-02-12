const DAY_NUM: &str = "10";

fn solve(input: &str) -> [String; 2] {
    let mut pipe_map: Vec<&[u8]> = input.lines()
        .map(std::str::as_bytes)
        .collect();
    let mut start = None;
    'find_start: for (r, line) in pipe_map.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == b'S' {
                start = Some((r, c));
                break 'find_start;
            }
        }
    }
    let start = start.expect("No start postion found");
    
    [
        "todo".to_string(),
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