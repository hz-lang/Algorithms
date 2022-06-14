//! 有向图的可达性。

use super::digraph::Digraph;

pub struct DirectedDFS {
    marked: Vec<bool>,
}

impl DirectedDFS {
    pub fn new(g: Digraph, s: usize) -> Self {
        let mut d = Self {
            marked: vec![false; g.v()],
        };
        d.dfs(&g, s);
        d
    }

    fn dfs(&mut self, g: &Digraph, s: usize) {
        self.marked[s] = true;
        for w in g.adj(s) {
            if !self.marked[*w] {
                self.dfs(g, *w);
            }
        }
    }

    /// 是否已经访问过该顶点。
    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let d = create();
        assert!(d.marked(1));
    }

    fn create() -> DirectedDFS {
        let mut g = Digraph::new(6);
        g.add_edge(1, 3);
        g.add_edge(3, 5);
        g.add_edge(5, 1);

        DirectedDFS::new(g, 1)
    }
}
