const DAY_NUM: &str = "8";

mod net{
    use std::{ops::Deref, str::Lines};

    #[derive(PartialEq)]
    #[repr(u8)]
    pub enum NodeType {
        Start([u8; 2]),
        Middle,
        End([u8; 2])
    }
    
    #[repr(C)]
    struct PtrNode {
        node_type: NodeType,
        left:  *const PtrNode,
        right: *const PtrNode
    }
    
    #[repr(C)]
    pub struct Node<'a> {
        pub node_type: NodeType,
        pub left:  &'a Node<'a>,
        pub right: &'a Node<'a>
    }
    
    #[allow(dead_code)]
    pub struct Net<'a>{
        nodes: Box<[Node<'a>]>,
        start: usize,
        all_starts: Box<[usize]>
    }
    
    impl NodeType {
        fn new(name: &str) -> Self {
            let label = name[0..2].as_bytes().try_into().unwrap();
            match &name[2..3] {
                "A" => NodeType::Start(label),
                "Z" => NodeType::End(label),
                _   => NodeType::Middle
            }
        }
    }
    
    impl PtrNode {
        fn new(name: &str) -> Self{
            let node_type = NodeType::new(name);
            use std::ptr;
            PtrNode{
                node_type,
                left:  ptr::null(),
                right: ptr::null()
            }
        }
    }
    
    impl<'a> Node<'a> {
        pub fn traverse(&self, directions: &str) -> &Self {
            let mut pos = self;
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
        pub fn new(map: Lines) -> Net<'a> {
            use std::collections::HashMap;
            let mut node_map = HashMap::new();
            let mut nodes = vec![];
            let mut direct_idx = vec![];
            for (idx, line) in map.enumerate() {
                let node  = &line[0..3];
                let left  = &line[7..10];
                let right = &line[12..15];
//                println!("{node} = ({left},{right})");
                node_map.insert(node, idx);
                nodes.push(PtrNode::new(node));
                direct_idx.push((left, right));
            }
            let mut start = None;
            let mut all_starts = vec![];
            for i in 0..nodes.len() {
                let (left, right) = direct_idx[i];
                nodes[i].left  = &nodes[node_map[&left]];
                nodes[i].right = &nodes[node_map[&right]];
                match nodes[i].node_type {
                    NodeType::Start([b'A', b'A']) => start = Some(i),
                    NodeType::Start(_) => all_starts.push(i),
                    _ => ()
                }
            }
            all_starts.push(start.expect("No start node found"));
            let all_starts = all_starts.into_boxed_slice();
            let nodes = nodes.into_boxed_slice();
            let nodes =  unsafe {
                use std::mem::transmute;
                transmute::<Box<[PtrNode]>, Box<[Node<'a>]>>(nodes)
            };
            let start = start.unwrap();
            Net{nodes, start, all_starts}
        }

        pub fn start(&self) -> &Node {
            &self[self.start]
        }
        
        pub fn all_starts(&self) -> Vec<&Node> {
            self.all_starts.iter().map(|i| &self[*i]).collect()
        }
    }
    
    impl<'a> Deref for Net<'a> {
        type Target = [Node<'a>];
        fn deref(&self) -> &Self::Target {
            &*self.nodes
        }
    }
}

fn all_at_ends(nodes: &[&net::Node]) -> bool {
    let at_end = |n: &&net::Node| {
        match (*n).node_type {
            net::NodeType::End(_) => true,
            _ => false
        }
    };
    nodes.into_iter().map(at_end).reduce(|a, n| a && n).unwrap()
}

fn solve(input: &str) -> [String; 2] {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();
    let network = net::Net::new(lines);
    let mut pos = network.start();
    let mut count = 0;
    while pos.node_type != net::NodeType::End(*b"ZZ") {
        pos = pos.traverse(directions);
        count += directions.len();
    }
    let mut pos_p2 = network.all_starts().into_boxed_slice();
    let mut count_p2 = 0;
    while !all_at_ends(&pos_p2) {
        for mut pos in pos_p2.iter_mut() {
            *pos = pos.traverse(directions);
        }
        count_p2 += directions.len();
    }
    [
        count.to_string(),
        count_p2.to_string()
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