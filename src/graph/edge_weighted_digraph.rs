//! 加权有向图。

use std::slice::Iter;

use super::directed_edge::DirectedEdge;

pub struct EdgeWeightedDigraph {
    v: usize,                    // 顶点的总数。
    e: usize,                    // 边的总数。
    adj: Vec<Vec<DirectedEdge>>, // 邻接表。
}

impl EdgeWeightedDigraph {
    /// Creates a new [`EdgeWeightedDigraph`].
    pub fn new(v: usize) -> Self {
        Self {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }

    /// 获取顶点的总数。
    pub fn v(&self) -> usize {
        self.v
    }

    /// 获取边的总数。
    pub fn e(&self) -> usize {
        self.e
    }

    /// 添加一条边。
    pub fn add_edge(&mut self, e: DirectedEdge) {
        self.adj[e.from()].push(e);
        self.e += 1;
    }

    /// 获取从 v 指出的边。
    pub fn adj(&self, v: usize) -> Iter<DirectedEdge> {
        self.adj[v].iter()
    }

    /// 获取所有的边。
    pub fn edges(&self) -> Edges {
        Edges::new(self.adj.iter())
    }
}

pub struct Edges<'a> {
    iter: Iter<'a, Vec<DirectedEdge>>,
    curr: Option<Iter<'a, DirectedEdge>>,
}

impl<'a> Edges<'a> {
    pub fn new(iter: Iter<'a, Vec<DirectedEdge>>) -> Self {
        Self { iter, curr: None }
    }
}

impl<'a> Iterator for Edges<'a> {
    type Item = &'a DirectedEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            if let Some(next) = self.iter.next() {
                self.curr = Some(next.iter());
            }
        }

        self.curr.as_mut().map_or(None, |i| i.next())
    }
}
