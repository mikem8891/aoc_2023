const DAY_NUM: &str = "11";

fn solve(input: &str) -> [String; 2] {
    let star_map: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for (i, row) in star_map.enumerate() {
        let empty = row.iter()
            .map(|s| s == b'.')
            .reduce(|acc, s| acc && s);
        if empty == Some(true) {
            expanded_rows.push(i)
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