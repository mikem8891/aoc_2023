use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let in_range = |r: usize, c: usize| { 
        c >= 0 && c < eng_schem[r].len() &&
        r >= 0 && r < eng_schem.len()
    };
    for num_row in 0..eng_schem.len() {
        let mut end = 0;
        while let Some(start) = eng_schem[num_row][end..].find(is_digit) {
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap();
            let mut part = 'find: {
                for c in ((start - 1)..(end + 1)).filter(|c| in_range(num_row - 1, *c)) {
                    if eng_schem[num_row - 1].as_bytes()[c] != b'.' {
                        break 'find true;
                    }
                }
                if in_range(num_row, start - 1) && eng_schem[num_row].as_bytes()[start - 1] != b'.' {
                    break 'find true;
                }
                if in_range(num_row, end) && eng_schem[num_row].as_bytes()[end] != b'.' {
                    break 'find true;
                }
                false
            };

        }
    }
    
    sum
}


#[cfg(test)]
mod test {
    
    #[test]
    fn example() {
        
    }
}