use cargo_snippet::snippet;

#[snippet("Graph")]
#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: i64,
}

#[snippet("Graph")]
impl Edge {
    pub fn new(to: usize, cost: i64) -> Edge {
        Edge { to: to, cost: cost }
    }
}

#[snippet("Graph")]
pub struct Graph {
    adjacency: Vec<Vec<Edge>>,
}

#[snippet("Graph")]
impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            adjacency: vec![vec![]; size],
        }
    }
    // 頂点数を取得
    pub fn len(&self) -> usize {
        self.adjacency.len()
    }
    // 有向辺を追加する
    pub fn add_directional_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.adjacency[from].push(Edge::new(to, cost));
    }
    // 無向辺を追加する
    pub fn add_undirectional_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.add_directional_edge(from, to, cost);
        self.add_directional_edge(to, from, cost);
    }
}

#[snippet("Graph")]
impl std::ops::Index<usize> for Graph {
    type Output = Vec<Edge>;
    fn index(&self, vertex: usize) -> &Self::Output {
        &self.adjacency[vertex]
    }
}
