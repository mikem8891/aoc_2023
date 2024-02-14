const DAY_NUM: &str = "11";

fn solve(input: &str) -> [String; 2] {
    let star_map: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let mut ex_rows = vec![];
    let mut ex_cols = vec![];
    for r in 0..star_map.len() {
        let empty = star_map[r].iter()
            .map(|s| s == b'.')
            .fold(true, |acc, s| acc && s);
        let new_last = ex_row.last().unwrap() +
            if empty {2} else {1};
        ex_rows.push(new_last);
        }
    }
    
    
    
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