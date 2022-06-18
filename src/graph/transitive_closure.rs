//! 顶点对可达性。

use super::{digraph::Digraph, directed_dfs::DirectedDFS};

pub struct TransitiveClosure {
    all: Vec<DirectedDFS>,
}

impl TransitiveClosure {
    pub fn new(g: &Digraph) -> Self {
        let mut t = Self { all: vec![] };
        for v in 0..g.v() {
            t.all.push(DirectedDFS::new(g, v));
        }
        t
    }

    /// v 到 w 是否可达。
    pub fn reachable(&self, v: usize, w: usize) -> bool {
        self.all[v].marked(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = create();
        assert!(t.reachable(1, 3));
    }

    fn create() -> TransitiveClosure {
        let mut g = Digraph::new(5);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        TransitiveClosure::new(&g)
    }
}