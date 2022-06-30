// 最小生成树的 Kruskal 算法。

use crate::sort::min_pq::MinPQ;
use std::collections::VecDeque;

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph, union_find::UF};

pub struct KruskalMST {
    mst: VecDeque<Edge>,
}

impl KruskalMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut k = Self {
            mst: VecDeque::new(),
        };

        let edges = g.edges();
        let mut pq = MinPQ::new(edges.len());
        for e in g.edges() {
            pq.insert(e.clone());
        }

        let mut uf = UF::new(g.v());
        while let Some(e) = pq.delete_min() {
            if k.mst.len() >= g.v() - 1 {
                break;
            }

            let v = e.either();
            if let Some(w) = e.other(v) {
                if !uf.connected(v, w) {
                    uf.union(v, w);
                    k.mst.push_back(e);
                }
            }
        }

        k
    }
}
