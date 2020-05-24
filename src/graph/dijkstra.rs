use cargo_snippet::snippet;

use crate::graph::graph::Graph;

#[snippet("dijkstra")]
pub fn dijkstra(g: &Graph, start: usize) -> Vec<i64> {
    use std::cmp::Reverse;
    let mut res = vec![std::i64::MAX; g.len()];
    res[start] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Reverse((0, start)));
    while let Some(Reverse((dist, now))) = heap.pop() {
        if res[now] == dist {
            for e in g[now].iter() {
                if res[now] + e.cost < res[e.to] {
                    res[e.to] = res[now] + e.cost;
                    heap.push(Reverse((res[e.to], e.to)));
                }
            }
        }
    }
    res
}

#[test]
fn test_dijkstra() {}
