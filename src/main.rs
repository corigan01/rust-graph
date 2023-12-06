use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::{self, Read},
};

struct Graph {
    nodes: Vec<Node>,
}

struct Node {
    id: usize,
    name: String,
    neignbors: Vec<(usize, i32, i32)>,
}

impl Node {
    fn new(id: usize, name: String, neignbors: Vec<(usize, i32, i32)>) -> Self {
        Self {
            id,
            name,
            neignbors,
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn ensure_node(&mut self, node: String) -> &mut Node {
        let id = self
            .nodes
            .iter()
            .find(|entry| &entry.name == &node)
            .map(|entry| entry.id);

        if let Some(id) = id {
            &mut self.nodes[id]
        } else {
            let id = self.nodes.len();
            self.nodes.push(Node::new(id, node, Vec::new()));
            &mut self.nodes[id]
        }
    }

    pub fn add_edge(&mut self, source: String, dest: String, weight: i32) {
        let dest_node = self.ensure_node(dest).id;
        let source_node = self.ensure_node(source);

        source_node.neignbors.push((dest_node, 0, weight));
    }

    pub fn lookup_node(&self, node: &str) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_, entry)| &entry.name == node)
            .map(|(id, _)| id)
    }

    pub fn node_name(&self, node: usize) -> Option<String> {
        self.nodes.get(node).map(|node| node.name.clone())
    }

    fn flow_in(&self, node: usize) -> (i32, i32) {
        let mut total_flow = 0;
        let mut total_capacity = 0;
        for (_, flow, capacity) in self
            .nodes
            .iter()
            .filter_map(|entry| entry.neignbors.iter().find(|element| element.0 == node))
        {
            total_flow += flow;
            total_capacity += capacity;
        }

        (total_flow, total_capacity)
    }

    fn flow_out(&self, node: usize) -> (i32, i32) {
        let node = &self.nodes[node];

        let mut total_flow = 0;
        let mut total_cap = 0;
        for (_, flow, cap) in node.neignbors.iter() {
            total_flow += flow;
            total_cap += cap;
        }

        (total_flow, total_cap)
    }

    fn breadth_first_search<Function>(
        &self,
        starting_node_id: usize,
        mut check_goal: Function,
    ) -> Option<Vec<usize>>
    where
        Function: FnMut(&Node) -> bool,
    {
        let mut follower_nodes = vec![None; self.nodes.len()];
        let mut queue = Vec::new();
        queue.push(starting_node_id);

        while let Some(last) = queue.pop() {
            let last = &self.nodes[last];

            if check_goal(last) {
                let mut path = Vec::new();
                let mut follower = Some(last.id);
                while follower.is_some() {
                    path.push(follower.unwrap());
                    follower = follower_nodes[follower.unwrap()];
                }
                return Some(path.into_iter().rev().collect());
            }

            for (child_id, flow, capacity) in last.neignbors.iter() {
                follower_nodes[*child_id] = Some(last.id);
                if flow < capacity {
                    queue.push(*child_id);
                }
            }
        }

        None
    }

    fn find_min_of_path(&self, path: &Vec<usize>) -> Option<i32> {
        path.windows(2)
            .map(|window| {
                let first = window[0];
                let second = window[1];

                self.nodes[first]
                    .neignbors
                    .iter()
                    .find_map(|(neignbor_index, flow, capacity)| {
                        if *neignbor_index == second {
                            Some(capacity - flow)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .min()
    }

    fn add_flow_in_path(&mut self, path: &Vec<usize>, addition: i32) {
        for window in path.windows(2) {
            let first = window[0];
            let second = window[1];

            let &mut (_, flow, capacity) = &mut self.nodes[first]
                .neignbors
                .iter_mut()
                .find(|(index, _, _)| *index == second)
                .unwrap();

            assert!(*flow + addition <= *capacity);
            *flow += addition;
        }
    }

    fn cal_flow(&mut self, starting_id: usize, ending_id: usize) -> i32 {
        while let Some(path) = self.breadth_first_search(starting_id, |node| node.id == ending_id) {
            println!("Path: {:?}", path);
            let min_flow = self.find_min_of_path(&path).unwrap();
            if min_flow == 0 {
                break;
            }

            self.add_flow_in_path(&path, min_flow);
            println!("{:?}", self);
        }

        self.flow_out(starting_id).0
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, node) in self.nodes.iter().enumerate() {
            f.write_fmt(format_args!("[{}]={}, ", id, node.name))?;
        }
        f.write_str("\n")?;
        for node in self.nodes.iter() {
            for (node_children, flow, weight) in node.neignbors.iter() {
                let child = &self.nodes[*node_children];
                f.write_fmt(format_args!(
                    "{} --> {} {:4} {:4}\n",
                    node.name, child.name, flow, weight
                ))?;
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut input_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open("sample.txt")?;

    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    println!("Input File: {input_string:#?}");

    let mut graph = Graph::new();
    for line in input_string.split("\n") {
        let mut line_split = line.split(",");
        match (
            line_split.next(),
            line_split.next(),
            line_split
                .next()
                .and_then(|value| value.parse::<i32>().ok()),
        ) {
            (Some(start), Some(end), Some(weight)) => {
                graph.add_edge(start.into(), end.into(), weight);
            }
            _ => (),
        }
    }

    println!("{graph:#?}");
    println!(
        "Flow: {}",
        graph.cal_flow(
            graph.lookup_node("s").unwrap(),
            graph.lookup_node("t").unwrap()
        )
    );

    Ok(())
}
