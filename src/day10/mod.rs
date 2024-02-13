const DAY_NUM: &str = "10";

trait Index2D<T> {
    fn get2D(&self, r: usize, c: usize) -> Option<&T>;
}

impl Index2D<T> for [&[T]] {
    fn get2D(&self, r: usize, c: usize) -> Option<&T> {
        self.get(r).map(|r| r.get(c)).flatten()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn thru_pipe(&self, pipe: u8) -> Self {
        use Direction as D;
        match self {
            D::Up => match pipe {
                b'|' => D::Up,
                b'7' => D::Left,
                b'F' => D::Right,
                pipe => panic!("Can't travel UP thru '{}'", pipe as char)
            },
            D::Left => match pipe {
                b'L' => D::Up,
                b'-' => D::Left
                b'F' => D::Down,
                pipe => panic!("Can't travel LEFT thru '{}'", pipe as char)
            },
            D::Right => match pipe {
                b'J' => D::Up,
                b'-' => D::Right,
                b'7' => D::Down,
                pipe => panic("Can't travel RIGHT thru '{}'", pipe as char)
            },
            D::Down => match pipe {
                b'J' => D::Left,
                b'L' => D::Right,
                b'|' => D::Down,
                pipe => panic("Can't travel DOWN thru '{}'", pipe as char)
            }
        }
    }
}

trait Move {
    fn move(&mut self, dir: Direction) -> &mut Self;
}

impl Move for (usize, usize) {
    fn move(&mut self, dir: Direction) -> &mut Self {
        let (r, c) = &mut self;
        use Direction as D;
        match dir {
            D::Up => *r -= 1,
            D::Left => *c -= 1,
            D::Right => *c += 1,
            D::Down => *r += 1
        }
        self
    }
}

fn solve(input: &str) -> [String; 2] {
    let mut pipe_map: Vec<&[u8]> = input.lines()
        .map(std::str::as_bytes)
        .collect();
    let mut start = None;
    'find_start: for (r, line) in pipe_map.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            if ch == b'S' {
                start = Some((r, c));
                break 'find_start;
            }
        }
    }
    let start = start.expect("No start postion found");
    let mut paths = vec![];
    let (r, c) = start;
    match pipe_map.get(r-1, c) {
        Some(b'|' | b'7' | b'F') => paths.push((r-1, c)),
        _ => ()
    }
    match pipe_map.get(r, c-1) {
        Some(b'-' | b'L' | b'F') => paths.push((r, c-1)),
        _ => ()
    }
    match pipe_map.get(r, c+1) {
        Some(b'-' | b'7' | b'J') => paths.push((r, c+1)),
        _ => ()
    }
    match pipe_map.get(r+1, c) {
        Some(b'|' | b'L' | b'J') => paths.push((r+1, c)),
        _ => ()
    }
    if paths.len() != 2 {
        panic!("Expected 2 paths. Found {} path(s).", paths.len());
    }
    let mut steps = 0;
    'outer: loop {
        steps += 1;
        for i in 0..2 {
            let (r, c) = paths[i]
            let pipe = pipe_map[r][c];
            path[i].move(pipe);
            if path[0] == path[1] {
                break 'outer;
            } else if path[i] == start {
                panic!("Paths did not converge");
            }
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