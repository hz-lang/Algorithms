//! 基于深度优先搜索的顶点排序。

use std::{iter::Rev, slice::Iter};

use super::digraph::Digraph;

pub struct DepthFirstOrder {
    marked: Vec<bool>,
    pre: Vec<usize>,          // 所有顶点的前序排列。
    post: Vec<usize>,         // 所有顶点的后序排列。
    reverse_post: Vec<usize>, // 所有顶点的逆后序排列。
}

impl DepthFirstOrder {
    pub fn new(g: Digraph) -> Self {
        let mut d = DepthFirstOrder {
            marked: vec![false; g.v()],
            pre: vec![],
            post: vec![],
            reverse_post: vec![],
        };
        for v in 0..g.v() {
            if !d.marked[v] {
                d.dfs(&g, v);
            }
        }
        d
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.pre.push(v);
        self.marked[v] = true;
        for w in g.adj(v) {
            if !self.marked[*w] {
                self.dfs(g, *w);
            }
        }
        self.post.push(v);
        self.reverse_post.push(v);
    }

    /// 获取前序排列。
    pub fn pre(&self) -> Iter<usize> {
        self.pre.iter()
    }

    /// 获取后序排列。
    pub fn post(&self) -> Iter<usize> {
        self.post.iter()
    }

    /// 获取逆后序排列。
    pub fn reverse_post(&self) -> Rev<Iter<usize>> {
        self.reverse_post.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_test() {
        let d = create();
        let pre = d.pre().as_slice();
        assert_eq!(&[0, 1, 2, 4, 3, 5, 6], pre);
    }

    #[test]
    fn post_test() {
        let d = create();
        let post = d.post().as_slice();
        assert_eq!(&[0, 4, 2, 5, 3, 1, 6], post);
    }

    #[test]
    fn reverse_post_test() {
        let d = create();
        let post: Vec<usize> = d.reverse_post().map(|i| *i).collect();
        assert_eq!(&[6, 1, 3, 5, 2, 4, 0], &post[..]);
    }

    fn create() -> DepthFirstOrder {
        let mut g = Digraph::new(7);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);
        g.add_edge(3, 5);
        DepthFirstOrder::new(g)
    }
}
