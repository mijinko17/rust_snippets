use cargo_snippet::snippet;

#[snippet("UnionFind")]
#[derive(Debug, Copy, Clone)]
enum UFdata {
    Child(usize),
    Root(usize),
}

#[snippet("UnionFind")]
struct UnionFind {
    data: Vec<UFdata>,
}

#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![UFdata::Root(1); n],
        }
    }
    fn root(&mut self, x: usize) -> usize {
        match self.data[x] {
            UFdata::Child(p) => {
                let res = self.root(p);
                self.data[x] = UFdata::Child(res);
                res
            }
            UFdata::Root(_) => x,
        }
    }
    pub fn size(&mut self, x: usize) -> usize {
        match self.data[x] {
            UFdata::Root(m) => m,
            UFdata::Child(_) => {
                let r = self.root(x);
                self.size(r)
            }
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (mut u, mut v) = (self.root(x), self.root(y));
        if u == v {
            false
        } else {
            if self.size(u) < self.size(v) {
                std::mem::swap(&mut u, &mut v);
            }
            self.data[u] = UFdata::Root(self.size(u) + self.size(v));
            self.data[v] = UFdata::Child(u);
            true
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(10);
    assert_eq!(uf.size(7), 1);
    uf.unite(2, 4);
    assert_eq!(uf.size(2), 2);
    uf.unite(4, 7);
    uf.unite(1, 8);
    uf.unite(4, 3);
    assert_eq!(uf.size(8), 2);
    assert_eq!(uf.size(3), 4);
    assert_eq!(uf.size(9), 1);
    assert!(uf.is_same(3, 7));
    assert!(!uf.is_same(8, 2));
    assert!(!uf.is_same(2, 5));
}
