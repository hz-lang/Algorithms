//! 深度优先搜索。

use super::Graph;

pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
}

impl DepthFirstSearch {
    /// 创建一个图的深度优先结果。
    pub fn new(g: Graph, s: i32) -> Self {
        let marked = vec![false; g.v()];
        let mut d = Self { marked, count: 0 };
        d.dfs(&g, s);
        d
    }

    /// 深度优先算法的实现。
    fn dfs(&mut self, g: &Graph, s: i32) {
        self.marked[s as usize] = true;
        self.count += 1;
        for w in g.adj(s) {
            if !self.marked(*w) {
                self.dfs(g, *w);
            }
        }
    }

    /// 获取指定的点是否连通。
    pub fn marked(&self, w: i32) -> bool {
        self.marked[w as usize]
    }

    /// 获取顶点数。
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test() {
        let g = graph();
        let d = DepthFirstSearch::new(g, 1);
        assert_eq!(6, d.count());
    }

    #[test]
    fn marked_test() {
        let g = graph();
        let d = DepthFirstSearch::new(g, 0);
        assert_eq!(true, d.marked(2));
    }

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