//! 环形检测。

use super::Graph;

pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
}

impl Cycle {
    pub fn new(g: Graph) -> Self {
        let marked = vec![false; g.v()];
        let mut c = Self {
            marked,
            has_cycle: false,
        };
        for s in 0..g.v() {
            if !c.marked[s] {
                c.dfs(&g, s, s);
            }
        }
        c
    }

    fn dfs(&mut self, g: &Graph, v: usize, u: usize) {
        self.marked[v] = true;
        for w in g.adj(v as i32) {
            if !self.marked[*w as usize] {
                self.dfs(g, *w as usize, v);
            } else if *w != u as i32 {
                self.has_cycle = true;
            }
        }
    }

    /// 图中是否有环。
    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph_data;
    use super::*;

    #[test]
    fn has_cycle_test() {
        let c = Cycle::new(graph_data());
        assert!(c.has_cycle());
    }
}
