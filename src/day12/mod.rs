const DAY_NUM: &str = "12";



fn permutations(line: &str) -> u32 {
    let (rec, dam_gp) = line.split_once(' ')
        .expect("no space on line");
    
    let dam_gp: Vec<u8> = dam_gp.split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    
    let total_dam: u8 = dam_gp.iter().sum();
    let known_dam = rec.bytes()
        .filter(|c| *c == b'#').count() as u8;
    let unknown_dam = total_dam - known_dam;
    
    let unknown_idx: Vec<_> = rec.match_indices('?')
        .map(|(i, _)| i).collect();
    let unknowns = unknown_idx.len() as u8;
    let unknown_op = unknowns - unknown_dam;
    
    let mut trial_rec = Vec::from(rec);
    struct State<'a>{
        dam_gp: &'a [u8],
        dam_rem:  u8,
        op_rem:   u8
    }
    fn perm_recur(rec: &mut [u8], state: &mut State) -> u32 {
        let i = rec.iter().position(|c| *c == b'?');
        if let Some(i) = i {
            // create permutations
            let mut count = 0;
            if state.dam_rem > 0 {
                state.dam_rem -= 1;
                rec[i] = b'#';
                count += perm_recur(rec, state);
                state.dam_rem += 1;
                rec[i] = b'?';
            }
            if state.op_rem > 0 {
                state.op_rem -= 1;
                rec[i] = b'.';
                count += perm_recur(rec, state);
                state.op_rem += 1;
                rec[i] = b'?';
            }
            assert!(state.dam_rem > 0 || state.op_rem > 0);
            count
        } else {
            // check permutation
            if check(rec, state.dam_gp) {1} else {0}
        }
    }
    fn check(rec: &[u8], dam_gp: &[u8]) -> bool {
        assert!(rec.iter().position(|c| *c == b'?').is_none());
        let mut i = 0;
        let mut trail_dam_gp = vec![];
        while let Some(j_s) = rec[i..].iter().position(|c| *c == b'#') {
            let j_s = j_s + i;
            let j_e = j_s + rec[j_s..].iter().position(|c| *c == b'.')
                .unwrap_or(rec.len() - j_s);
            i = j_e;
            assert!(j_e > j_s, "j_e > j_s, {j_e} > {j_s}");
            trail_dam_gp.push((j_e - j_s) as u8);
        }
        dam_gp.iter().zip(&trail_dam_gp).all(|(a, b)| a == b)
    }
    let mut state = State {dam_gp: &dam_gp, dam_rem: unknown_dam, op_rem: unknown_op};
    perm_recur(&mut trial_rec, &mut state)
}

fn solve(input: &str) -> [String; 2] {
    let total_permutations: u32 = input.lines().map(permutations).sum();
    
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