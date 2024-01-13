use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let is_in_range = |r: usize, c: usize| { 
        c >= 0 && c < eng_schem[r].len() &&
        r >= 0 && r < eng_schem.len()
    };
    let is_part_at = |r, c| {
        is_in_range(r, c) &&
        eng_schem[r].as_bytes()[c] != b'.')
    };
    for num_row in 0..eng_schem.len() {
        let mut end = 0;
        while let Some(start) = eng_schem[num_row][end..].find(is_digit) {
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap();
            let mut part = 'find: {
                let ranges = [
                    (num_row - 1, (start - 1)..(end + 1)),
                    (num_row    , (start - 1)..(start  )),
                    (num_row    , (end      )..(end + 1)),
                    (num_row + 1, (start - 1)..(end + 1))
                ];
                for (r, c_range) in ranges {
                    for c in c_range {
                        if let Some(b) = eng_schem.get(r).map(|row| row.get(c)).flatten() {
                            if b != b'.' {
                                break 'find true;
                            }
                        }
                    }
                }
                false
                for c in ((start - 1)..(end + 1)).filter(|c| in_range(num_row - 1, *c)) {
                    if eng_schem[num_row - 1].as_bytes()[c] != b'.' {
                        break 'find true;
                    }
                }
                if (in_range(num_row, start - 1) && 
                    eng_schem[num_row].as_bytes()[start - 1] != b'.') ||
                    (in_range(num_row, end) && 
                    eng_schem[num_row].as_bytes()[end] != b'.') {
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