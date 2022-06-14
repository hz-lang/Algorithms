//! 有向环。

use std::{iter::Rev, slice::Iter};

use super::digraph::Digraph;

pub struct DirectedCycle {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    cycle: Vec<usize>,
    on_stack: Vec<bool>,
}

impl DirectedCycle {
    /// 创建一个 [`DirectedCycle`]。
    pub fn new(g: Digraph) -> Self {
        let mut d = Self {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            cycle: vec![],
            on_stack: vec![false; g.v()],
        };
        for v in 0..g.v() {
            if !d.marked[v] {
                d.dfs(&g, v);
            }
        }
        d
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for w in g.adj(v) {
            if self.has_cycle() {
                return;
            }
            if !self.marked[*w] {
                self.edge_to[*w] = v;
                self.dfs(g, *w);
            } else if self.on_stack[*w] {
                self.cycle.clear();
                let mut x = v;
                while x != *w {
                    self.cycle.push(x);
                    x = self.edge_to[x];
                }
                self.cycle.push(*w);
                self.cycle.push(v);
            }
        }
        self.on_stack[v] = false;
    }

    /// 是否有环。
    pub fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }

    /// 获取有向环中的所有顶点。
    pub fn cycle(&self) -> Rev<Iter<usize>> {
        self.cycle.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_cycle_test() {
        let d = create();
        assert!(d.has_cycle());
    }

    #[test]
    fn cycle_test() {
        let d = create();
        let mut c = d.cycle();
        assert_eq!(Some(&3), c.next());
        assert_eq!(Some(&1), c.next());
        assert_eq!(Some(&2), c.next());
        assert_eq!(Some(&3), c.next());
        assert_eq!(None, c.next());
    }

    fn create() -> DirectedCycle {
        let mut g = Digraph::new(5);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
        DirectedCycle::new(g)
    }
}
