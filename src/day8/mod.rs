const DAY_NUM: &str = "8";

struct Node {
    left:  *const Node,
    right: *const Node
}

impl Node {
    fn traverse(directions: &str) {
        
    }
}

fn solve(input: &str) -> [String; 2] {


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