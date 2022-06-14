//! 深度优先搜索。

use super::Graph;

pub struct DepthFirstSearch {
    marked: Vec<bool>,   // 是否已访问过。
    edge_to: Vec<usize>, // 起点到某顶点的最后一个顶点。
    s: usize,            // 起点。
}

impl DepthFirstSearch {
    /// 创建一个图的深度优先结果。
    pub fn new(g: Graph, s: usize) -> Self {
        let marked = vec![false; g.v()];
        let edge_to = vec![0; g.v()];
        let mut d = Self { marked, edge_to, s };
        d.dfs(&g, s);
        d
    }

    /// 深度优先算法。
    fn dfs(&mut self, g: &Graph, s: usize) {
        self.marked[s] = true;
        for w in g.adj(s) {
            if !self.marked[*w] {
                self.edge_to[*w] = s;
                self.dfs(g, *w);
            }
        }
    }

    /// 是否存在 s 到 v 的路径。
    pub fn has_path_to(&self, w: usize) -> bool {
        self.marked[w]
    }

    /// 获取 s 到 v 的路径。
    pub fn path_to(&self, v: usize) -> Vec<usize> {
        if !self.has_path_to(v) {
            return vec![];
        }

        let mut path = vec![];
        let mut x = v;
        while x != self.s {
            path.push(x);
            x = self.edge_to[x];
        }
        path.push(self.s);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::graph_data;

    #[test]
    fn has_path_to_test() {
        let g = graph_data();
        let d = DepthFirstSearch::new(g, 1);
        assert!(d.has_path_to(0));
    }

    #[test]
    fn path_to_test() {
        let g = graph_data();
        let d = DepthFirstSearch::new(g, 0);
        let mut list = d.path_to(1);
        assert_eq!(Some(0), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }
}
