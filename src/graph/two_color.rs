//! 双色问题。

use super::Graph;

pub struct TwoColor {
    marked: Vec<bool>,
    color: Vec<bool>,
    is_two_colorable: bool,
}

impl TwoColor {
    pub fn new(g: Graph) -> Self {
        let marked = vec![false; g.v()];
        let color = vec![false; g.v()];
        let mut c = Self {
            marked,
            color,
            is_two_colorable: true,
        };
        for s in 0..g.v() {
            if !c.marked[s] {
                c.dfs(&g, s);
            }
        }
        c
    }

    fn dfs(&mut self, g: &Graph, s: usize) {
        self.marked[s] = true;
        for w in g.adj(s as i32) {
            if !self.marked[*w as usize] {
                self.color[*w as usize] = !self.color[*w as usize];
                self.dfs(g, *w as usize);
            } else if self.color[s] == self.color[*w as usize] {
                self.is_two_colorable = false;
            }
        }
    }

    /// 是否是二分图。
    pub fn is_bipartite(&self) -> bool {
        self.is_two_colorable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::graph_data;

    #[test]
    fn test() {
        let c = TwoColor::new(graph_data());
        assert!(!c.is_bipartite());
    }
}
