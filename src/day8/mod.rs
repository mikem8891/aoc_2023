const DAY_NUM: &str = "8";

mod net{
    
    #[repr(u8)]
    enum NodeType {
        Start,
        Middle,
        End
    }
    
    #[repr(C)]
    struct PtrNode {
        node_type: NodeType,
        left:  *const Node,
        right: *const Node
    }
    
    #[repr(C)]
    pub struct Node<'a> {
        node_type: NodeType,
        pub left:  &<'a> Node<'a>,
        pub right: &<'a> Node<'a>
    }
    
    pub struct Net<'a>{
        nodes: Box<[Node<'a>]>,
        start: &'a Node
    }
    
    impl NodeType {
        fn new(name: &str) {
            match name {
                "AAA" => NodeType::Start,
                "ZZZ" => NodeType::End,
                _     => NodeType::Middle
            }
        }
    }
    
    impl PtrNode {
        fn new(name: &str){
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
            let (mut start, mut end) = (None, None);
            for i in 0..net.len() {
                let (left, right) = direct_idx[i];
                net[i].left  = &net[node_map[left]];
                net[i].right = &net[node_map[right]];
                match net[i].node_type {
                    NodeType::Start => start = Some(i);,
                    NodeType::End   => end   = Some(i);,
                    NodeType::Middle => ()
                }
            }
            let net = net.into_boxed_slice();
            let nodes = unsafe {
                use std::mem::transmute;
                transmute::<Box<[PtrNode]>, Box<[Node]>>(net)
            };
            let start = &net[start.expect("No start node found")];
            end.expect("No end node found");
            Net{nodes, start}
        }
    }
    
    impl<'a> Deref for Net<'a> {
        type Target = [Node];
        fn deref(&self) -> &Self::Target {
            &*self.nodes
        }
    }
}

fn solve(input: &str) -> [String; 2] {
    let lines = input.lines();
    let directions = lines.next().unwrap();
    while lines.next().unwrap().trim().is_empty(){}
    let network = net::Net::new(lines.collect::<&str>());
    let mut pos = network.start();
    let mut count = 0;
    while pos.node_type != NodeType::End {
        pos = pos.traverse(directions);
        count += directions.len();
    }
    [
        count.to_string(),
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