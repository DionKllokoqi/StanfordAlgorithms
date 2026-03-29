use stanford_algorithms::assignment4::Graph;
use std::collections::HashMap;
use std::fs;
use std::io;

fn load_graph_data() -> io::Result<HashMap<u32, Vec<u32>>> {
    let contents = fs::read_to_string("data/karger_min_cut.txt")?;
    let data: HashMap<u32, Vec<u32>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1..].to_vec())
        })
        .collect();

    Ok(data)
}

fn main() -> io::Result<()> {
    let data = load_graph_data()?;
    let graph = Graph::from_adjacency_map(&data);

    let min_cut = graph.get_min_cut(10000);

    println!("min_cut: {:?}", min_cut.len());

    Ok(())
}
