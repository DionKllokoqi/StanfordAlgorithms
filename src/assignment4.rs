use rand::RngExt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Graph {
    pub vertices: Vec<Vec<u32>>,
    pub edges: Vec<(u32, u32)>,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
    }
}

impl Graph {
    pub fn from_adjacency_map(data: &HashMap<u32, Vec<u32>>) -> Self {
        let vertices: Vec<Vec<u32>> = data.keys().map(|&k| vec![k]).collect();

        let mut edges = Vec::new();
        for (&key, neighbors) in data {
            for &neighbor in neighbors {
                if neighbor > key {
                    edges.push((key, neighbor));
                }
            }
        }

        Graph { vertices, edges }
    }

    pub fn get_min_cut(&self, iterations: usize) -> Vec<(u32, u32)> {
        let vertex_to_idx: HashMap<u32, usize> = self
            .vertices
            .iter()
            .enumerate()
            .map(|(i, g)| (g[0], i))
            .collect();

        let mut best_cut: Option<Vec<(u32, u32)>> = None;

        for _ in 0..iterations {
            let cut = self.run_single_min_cut(&vertex_to_idx);

            if best_cut.as_ref().is_none_or(|best| cut.len() < best.len()) {
                best_cut = Some(cut);
            }
        }

        best_cut.unwrap()
    }

    fn run_single_min_cut(&self, vertex_to_idx: &HashMap<u32, usize>) -> Vec<(u32, u32)> {
        let mut rng = rand::rng();
        let mut edges = self.edges.clone();
        let mut uf = UnionFind::new(self.vertices.len());
        let mut num_components = self.vertices.len();

        while num_components > 2 {
            let idx = rng.random_range(0..edges.len());
            let (left, right) = edges[idx];

            let left_root = uf.find(vertex_to_idx[&left]);
            let right_root = uf.find(vertex_to_idx[&right]);

            if left_root == right_root {
                edges.swap_remove(idx);
                continue;
            }

            uf.union(left_root, right_root);
            num_components -= 1;
            edges.swap_remove(idx);
        }

        let groups: HashMap<u32, usize> = vertex_to_idx
            .iter()
            .map(|(&v, &idx)| (v, uf.find(idx)))
            .collect();

        edges.retain(|&(left, right)| groups[&left] != groups[&right]);

        edges
    }
}
