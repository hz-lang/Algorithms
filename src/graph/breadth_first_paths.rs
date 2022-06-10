//! 广度优先搜索。

use std::collections::VecDeque;

use super::Graph;

pub struct BreadthFirstPaths {
    marked: Vec<bool>, // 是否已访问过。
    edge_to: Vec<i32>, // 到顶点的路径上最后一个顶点。
    s: i32,            // 起点。
}

impl BreadthFirstPaths {
    /// 创建图的广度优先结果。
    pub fn new(g: Graph, s: i32) -> Self {
        let marked = vec![false; g.v()];
        let edge_to = vec![0; g.v()];
        let mut d = Self { marked, edge_to, s };
        d.bfs(&g, s);
        d
    }

    /// 广度优先算法。
    fn bfs(&mut self, g: &Graph, s: i32) {
        let mut queue = VecDeque::new();
        queue.push_back(s);

        self.marked[s as usize] = true;
        while let Some(value) = queue.pop_front() {
            for w in g.adj(value) {
                if !self.marked[*w as usize] {
                    self.marked[*w as usize] = true; // 找到最短路径。
                    self.edge_to[*w as usize] = value; // 最短路径的最后一条边。
                    queue.push_back(*w); // 添加最短路径。
                }
            }
        }
    }

    /// 是否存在 s 到 v 的路径。
    pub fn has_path_to(&self, v: i32) -> bool {
        self.marked[v as usize]
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
        let d = BreadthFirstPaths::new(g, 1);
        assert!(d.has_path_to(0));
    }

    #[test]
    fn path_to_test() {
        let g = graph();
        let d = BreadthFirstPaths::new(g, 0);
        let mut list = d.path_to(1);
        assert_eq!(Some(0), list.pop());
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
