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
    
    for cond in rec.as_bytes() {
        
    }
    
    let total_dam = dam_gp.iter().sum();
    let known_dam = rec.bytes()
        .filter(|c| c == b'#').count();
    let unknown_dam = total_dam - known_dam;
    
    let unknown_idx: Vec<_> = rec.match_indices('?')
        .map(|(i, _)| i).collect();
    let unknowns = unknown_idx.len();
    
    let mut trial_rec = String::new(rec);
    let mut count = 0;
    let mut dam_cnt = 0;
    let check = |i: usize| {
        let i = trial_rec.find('?')
        if let Some(i) = i {
            let c = trial_rec.as_bytes_mut()[i];
            c = b'.';
            check(i + 1);
            if dam_cnt < unknown_dam {
                let c = trial_rec.as_bytes_mut()[i];
                c = b'#';
                dam_cnt += 1;
                check(i + 1);
                dam_cnt -= 1;
            }
            let c = trial_rec.as_bytes_mut()[i];
            c = b'?';
        } else {
            
        }
    }
    
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