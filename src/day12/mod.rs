const DAY_NUM: &str = "12";



fn permutations(line: &str) -> u32 {
    let (rec, dam_gp) = line.split_once(' ')
        .expect("no space on line");
    
    let dam_gp: Vec<u8> = dam_gp.split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    
    let total_dam = dam_gp.iter().sum();
    let known_dam = rec.bytes()
        .filter(|c| *c == b'#').count();
    let unknown_dam = total_dam - known_dam;
    
    let unknown_idx: Vec<_> = rec.match_indices('?')
        .map(|(i, _)| i).collect();
    let unknowns = unknown_idx.len();
    let unknown_op = unknowns - unknown_dam;
    
    let mut trial_rec = Vec::from(rec);
    let mut count = 0;
    let (mut dam_cnt, mut op_cnt) = (0, 0);
    fn is(that: u8) {|this: u8| this == that}
    struct State{
        dam_gp: &[u8],
        dam_rem:  u8,
        op_rem:   u8
    }
    fn perm_recur(rec: &mut [u8], state: &mut State) -> u32 {
        let i = rec.iter().position(|c| c == b'?');
        if let Some(i) = i {
            // create permutations
            if *state.dam_rem > 0 {
                *dam_rem -= 1;
                rec[i] = b'#';
                perm_recur(rec, state);
                *dam_rem += 1;
                rec[i] = b'?';
            }
            if op_rem > 0 {
                *op_rem -= 1;
                rec[i] = b'.';
                perm_recur(rec, state);
                *op_rem += 1;
                rec[i] = b'?';
            }
            assert!(dam_rem > 0 || op_rem > 0);
        } else {
            // check permutation
            if 
        }
    }
    let check = |i: usize| {
        let of = |a: u8| {|b: u8| a == b};
        let i = trial_rec.iter().position(of(b'?'));
        if let Some(i) = i {
            assert!(op_cnt < unknown_op  || dam_cnt < unknown_dam);
            if op_cnt < unknown_op {
                unsafe{*trial_rec.as_bytes_mut()[i] = b'.';}
                op_cnt += 1;
                check(i + 1);
                op_cnt -= 1;
            }
            if dam_cnt < unknown_dam {
                unsafe{*trial_rec.as_bytes_mut()[i] = b'#';}
                dam_cnt += 1;
                check(i + 1);
                dam_cnt -= 1;
            }
            unsafe{*trial_rec.as_bytes_mut()[i] = b'?';}
        } else {
            assert!(trial_rec.find('?').is_none());
            let mut i = 0;
            let trail_dam_gp = vec![];
            while let Some(j_s) = trial_rec[i..].find('#') {
                let j_e = trial_rec[j_s..].find('.')
                    .unwrap_or(trial_rec.len());
                trail_dam_gp.push(j_e - j_s);
            }
            assert!(trail_dam_gp.sum() == total_dam);
            if dam_gp.iter().zip(&trail_dam_gp).all(|(a, b)| a == b) {
                count += 1;
            }
        }
    };
    check(0);
    count
}

fn solve(input: &str) -> [String; 2] {
    let total_permutations = input.lines().map(permutations).sum();
    
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