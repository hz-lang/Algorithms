//! 最小生成树 Prim 的即时实现。

use super::{super::sort::min_pq::MinPQ, edge::Edge, edge_weighted_graph::EdgeWeightedGraph};

pub struct PrimMST {
    marked: Vec<bool>,    // 是否访问过。
    edge_to: Vec<Edge>,   // 离树最近的边。
    dist_to: Vec<f64>,    // edge_to[i].weight()。
    pq: MinPQ<EdgeIndex>, // 有效的横切边。
}

impl PrimMST {
    /// 创建新的 [`LazyPrimMST`]。
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut l = Self {
            marked: vec![false; g.v()],
            edge_to: Vec::with_capacity(g.v()),
            dist_to: vec![f64::INFINITY; g.v()],
            pq: MinPQ::new(g.v()),
        };

        l.dist_to[0] = 0.0;
        l.pq.insert(EdgeIndex::new(0, 0.0));

        // 获取权重最小的边。
        while let Some(ei) = l.pq.delete_min() {
            l.visit(g, ei.v);
        }

        l
    }

    fn visit(&mut self, g: &EdgeWeightedGraph, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            e.other(v).map(|w| {
                if self.marked[w] || e.weight() >= self.dist_to[w] {
                    return; // v-w 失效。
                }

                // 连接 w 和树的最佳边。
                self.edge_to[w] = e.clone();
                self.dist_to[w] = e.weight();
                match self.pq.get_mut(|v| v.v == w) {
                    Some(v) => v.weight = self.dist_to[w],
                    None => self.pq.insert(EdgeIndex::new(w, self.dist_to[w])),
                }
            });
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct EdgeIndex {
    v: usize,
    weight: f64,
}

impl EdgeIndex {
    fn new(v: usize, weight: f64) -> Self {
        Self { v, weight }
    }
}

impl Eq for EdgeIndex {}
impl Ord for EdgeIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect(&format!("{:?} 不能与 {:?} 比较", self, other))
    }
}
