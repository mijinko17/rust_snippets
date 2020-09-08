#[derive(Debug, Copy, Clone)]
enum DsuNode {
    Leader(usize),
    Child(usize),
}

pub struct Dsu {
    len: usize,
    nodes: Vec<DsuNode>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            len: n,
            nodes: vec![DsuNode::Leader(1); n],
        }
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            x
        } else {
            let (size_x, size_y) = (self.size(x), self.size(y));
            if size_x < size_y {
                std::mem::swap(&mut x, &mut y);
            }
            self.nodes[x] = DsuNode::Leader(size_x + size_y);
            self.nodes[y] = DsuNode::Child(x);
            x
        }
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        match self.nodes[a] {
            DsuNode::Child(p) => {
                let res = self.leader(p);
                self.nodes[a] = DsuNode::Child(res);
                res
            }
            DsuNode::Leader(_) => a,
        }
    }
    pub fn size(&mut self, a: usize) -> usize {
        match self.nodes[a] {
            DsuNode::Leader(m) => m,
            DsuNode::Child(_) => {
                let r = self.leader(a);
                self.size(r)
            }
        }
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut result = self
            .nodes
            .iter()
            .map(|&node| {
                Vec::with_capacity(match node {
                    DsuNode::Child(_) => 0,
                    DsuNode::Leader(sz) => sz,
                })
            })
            .collect::<Vec<_>>();
        for i in 0..self.len {
            result[self.leader(i)].push(i);
        }
        result
            .iter()
            .filter(|gr| !gr.is_empty())
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[test]
fn test_dsu() {
    let mut dsu = Dsu::new(10);
    assert_eq!(dsu.size(7), 1);
    dsu.merge(2, 4);
    assert_eq!(dsu.size(2), 2);
    dsu.merge(4, 7);
    dsu.merge(1, 8);
    dsu.merge(4, 3);
    assert_eq!(dsu.size(8), 2);
    assert_eq!(dsu.size(3), 4);
    assert_eq!(dsu.size(9), 1);
    assert!(dsu.same(3, 7));
    assert!(!dsu.same(8, 2));
    assert!(!dsu.same(2, 5));
}
