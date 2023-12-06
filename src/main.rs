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
    ) -> Vec<usize>
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
                let mut follower = Some(starting_node_id);
                while follower.is_some() {
                    path.push(follower.unwrap());
                    follower = follower_nodes[follower.unwrap()];
                }
                return path;
            }

            for (child_id, flow, capacity) in last.neignbors.iter() {
                follower_nodes[last.id] = Some(*child_id);
                if flow < capacity {
                    queue.push(*child_id);
                }
            }
        }

        Vec::new()
    }

    fn find_min_of_path(&self, path: Vec<usize>) -> i32 {
        todo!()
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
        "Flow-out of 's': {:?}",
        graph.flow_out(graph.lookup_node("s").unwrap())
    );

    let t_node_id = graph.lookup_node("t").unwrap();
    let path_from_s_to_t =
        graph.breadth_first_search(graph.lookup_node("s").unwrap(), |node| node.id == t_node_id);

    print!("Path from 's' to 't':\n\t");
    for (index, child) in path_from_s_to_t.iter().enumerate() {
        print!("{}", graph.node_name(*child).unwrap());

        if index < path_from_s_to_t.len() - 1 {
            print!(" -> ");
        }
    }

    println!();

    Ok(())
}
