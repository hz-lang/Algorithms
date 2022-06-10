//! 深度优先搜索。

use super::Graph;

pub struct DepthFirstSearch {
    marked: Vec<bool>, // 是否已访问过。
    edge_to: Vec<i32>, // 起点到某顶点的最后一个顶点。
    s: i32,            // 起点。
}

impl DepthFirstSearch {
    /// 创建一个图的深度优先结果。
    pub fn new(g: Graph, s: i32) -> Self {
        let marked = vec![false; g.v()];
        let edge_to = vec![0; g.v()];
        let mut d = Self { marked, edge_to, s };
        d.dfs(&g, s);
        d
    }

    /// 深度优先算法。
    fn dfs(&mut self, g: &Graph, s: i32) {
        self.marked[s as usize] = true;
        for w in g.adj(s) {
            if !self.marked[*w as usize] {
                self.edge_to[*w as usize] = s;
                self.dfs(g, *w);
            }
        }
    }

    /// 是否存在 s 到 v 的路径。
    pub fn has_path_to(&self, w: i32) -> bool {
        self.marked[w as usize]
    }

    /// 获取 s 到 v 的路径。
    pub fn path_to(&self, v: i32) -> Vec<i32> {
        if !self.has_path_to(v) {
            return vec![];
        }

        let mut path = vec![];
        let mut x = v;
        while x != self.s {
            path.push(x);
            x = self.edge_to[x as usize];
        }
        path.push(self.s);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_path_to_test() {
        let g = graph();
        let d = DepthFirstSearch::new(g, 1);
        assert!(d.has_path_to(0));
    }

    #[test]
    fn path_to_test() {
        let g = graph();
        let d = DepthFirstSearch::new(g, 0);
        let mut list = d.path_to(1);
        assert_eq!(Some(0), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }

    // 0 ------ 2
    // |\      /|\
    // | \    / | \
    // |    1   |  \
    // 5 ------ 3 - 4
    fn graph() -> Graph {
        let mut g = Graph::new(6);
        g.add_edge(0, 2);
        g.add_edge(0, 1);
        g.add_edge(0, 5);
        g.add_edge(1, 0);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        g.add_edge(2, 1);
        g.add_edge(2, 3);
        g.add_edge(2, 4);
        g.add_edge(3, 5);
        g.add_edge(3, 4);
        g.add_edge(3, 2);
        g.add_edge(4, 3);
        g.add_edge(4, 2);
        g.add_edge(5, 3);
        g.add_edge(5, 0);
        g
    }
}
