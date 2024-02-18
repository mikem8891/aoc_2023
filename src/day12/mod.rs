const DAY_NUM: &str = "12";

enum Condition {
    Operational, Damaged, Unknown
}

impl From<u8> for Condition {
    fn from(cond: u8) -> Self {
        use Condition as C;
        match cond{
            b'.' => C::Operational,
            b'#' => C::Damaged,
            b'?' => C::Unknown,
            err => panic!("expected '.', '#', '?'; found '{}'", err as char)
        }
    }
}

fn permutations(line: &str) -> u32 {
    let (rec, dam_gp) = line.split_once(' ')
        .expect("no space on line");
    
    let dam_gp: Vec<u8> = dam_gp.split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    
    let total_dam = dam_gp.iter().sum();
    let known_dam = rec.bytes()
        .filter(|c| c == b'#').count();
    let unknown_dam = total_dam - known_dam;
    
    todo!();
}

fn solve(input: &str) -> [String; 2] {
    let total_permutations = input.lines().map(permutations).sum();
    
    [
        total_permutations.to_string(),
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