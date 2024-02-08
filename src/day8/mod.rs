const DAY_NUM: &str = "8";

mod net{
    struct PtrNode {
        left:  *const Node,
        right: *const Node
    }
    
    pub struct Node<'a> {
        pub left:  &<'a> Node<'a>,
        pub right: &<'a> Node<'a>
    }
    
    pub struct Net<'a>(Box<[Node]>)
    
    impl PtrNode {
        fn new(){
            use std::ptr;
            PtrNode{
                left:  ptr::null(),
                right: ptr::null()
            }
        }
    }
    
    impl<'a> Node<'a> {
        fn traverse(&self, directions: &str) -> &Self {
            let mut pos = &self;
            for turn in directions.chars() {
                pos = match turn {
                    'L' => pos.left,
                    'R' => pos.right,
                    _ => panic!()
                };
            }
            pos
        }
    }
    
    impl<'a> Net<'a> {
        pub fn new<'a>(map: &str) -> Net<'a> {
            use std::collections::HashMap;
            let mut node_map = HashMap::new();
            let mut net = vec![];
            let mut direct_idx = vec![];
            for (idx, line) in map.lines().enumerate() {
                let node  = line[0..3];
                let left  = line[7..10];
                let right = line[12..15];
                node_map.insert(node, idx);
                net.push(PtrNode::new(node));
                direct_idx.push((left, right));
            }
            for i in 0..net.len() {
                let (left, right) = direct_idx[i];
                net[i].left  = &net[node_map[left]];
                net[i].right = &net[node_map[right]];
            }
            let net = net.into_boxed_slice();
            let net = unsafe {
                use std::mem::transmute;
                transmute::<Box<[PtrNode]>, Box<[Node]>(net)
            }
            Net(net)
        }
    }
    
    impl<'a> Deref for Net<'a> {
        type Target = [Node];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
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