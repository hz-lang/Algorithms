//! 加权无向图。

use super::edge::Edge;
use std::{ptr::NonNull, slice::Iter};

type T = Option<NonNull<Edge>>;

pub struct EdgeWeightedGraph {
    v: usize,         // 顶点的总数。
    e: usize,         // 边的总数。
    adj: Vec<Vec<T>>, // 邻接表。
}

impl EdgeWeightedGraph {
    /// 创建一个新的 [`EdgeWeightedGraph`]。
    pub fn new(v: usize) -> Self {
        Self {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }

    /// 图的顶点数。
    pub fn v(&self) -> usize {
        self.v
    }

    /// 图的边数。
    pub fn e(&self) -> usize {
        self.e
    }

    /// 添加一条边。
    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let e = Box::leak(Box::new(e)).into();
        self.adj[v].push(Some(e));

        let temp_e = unsafe { e.as_ref() };
        if let Some(w) = temp_e.other(v) {
            self.adj[w].push(Some(e));
        }

        self.e += 1;
    }

    /// 获取与 v 相关联的所有边。
    pub fn adj(&self, v: usize) -> EdgeWeightedGraphIter {
        EdgeWeightedGraphIter {
            it: self.adj[v].iter()
        }
    }

    /// 获取图的所有边。
    pub fn edges(&self) -> Vec<&Edge> {
        let mut b = vec![];
        for v in 0..self.v {
            for e in self.adj(v) {
                if e.other(v).filter(|&i| i > v).is_some() {
                    b.push(e);
                }
            }
        }
        b
    }
}

pub struct EdgeWeightedGraphIter<'a> {
    it: Iter<'a, T>,
}

impl<'a> Iterator for EdgeWeightedGraphIter<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        match self.it.next() {
            None => None,
            Some(e) => e.map(|i| unsafe { i.as_ref() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let e = create();
        let list: Vec<Edge> = e.edges().iter().map(|i| (*i).clone()).collect();
        let r = edges();
        assert_eq!(r, list);
    }

    fn create() -> EdgeWeightedGraph {
        let mut g = EdgeWeightedGraph::new(5);
        for e in edges() {
            g.add_edge(e);
        }
        g
    }

    fn edges() -> Vec<Edge> {
        vec![
            Edge::new(1, 2, 0.5),
            Edge::new(1, 4, 0.7),
            Edge::new(2, 3, 0.6),
        ]
    }
}
