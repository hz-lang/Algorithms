//! 最小生成树 Prim 的延时实现。

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph};
use std::{cmp::Reverse, collections::BinaryHeap, iter::Rev, slice::Iter};

pub struct LazyPrimMST {
    marked: Vec<bool>,             // 树的顶点。
    mst: Vec<Edge>,                // 树的边。
    pq: BinaryHeap<Reverse<Edge>>, // 横切边（含失效的边）。
}

impl LazyPrimMST {
    /// 创建新的 [`LazyPrimMST`]。
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut l = Self {
            marked: vec![false; g.v()],
            mst: vec![],
            pq: BinaryHeap::new(),
        };

        l.visit(g, 0); // 假设 g 连通。

        // 获取权重最小的边。
        while let Some(Reverse(e)) = l.pq.pop() {
            let v = e.either();
            if let Some(w) = e.other(v) {
                if l.marked[v] && l.marked[w] {
                    continue;
                }

                l.mst.push(e);

                if !l.marked[v] {
                    l.visit(g, v);
                }
                if !l.marked[w] {
                    l.visit(g, w);
                }
            }
        }

        l
    }

    fn visit(&mut self, g: &EdgeWeightedGraph, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            if e.other(v).filter(|w| !self.marked[*w]).is_some() {
                self.pq.push(Reverse(e.clone()));
            }
        }
    }

    /// 获取树的边。
    pub fn edges(&self) -> Rev<Iter<Edge>> {
        self.mst.iter().rev()
    }
}
