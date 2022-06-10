//! 连通分量。

use super::Graph;

pub struct CC {
    marked: Vec<bool>,
    id: Vec<i32>,
    count: usize,
}

impl CC {
    pub fn new(g: Graph) -> Self {
        let marked = vec![false; g.v()];
        let id = vec![0; g.v()];
        let mut c = Self {
            marked,
            id,
            count: 0,
        };
        for s in 0..g.v() {
            if !c.marked[s] {
                c.dfs(&g, s);
                c.count += 1;
            }
        }
        c
    }

    fn dfs(&mut self, g: &Graph, s: usize) {
        self.marked[s] = true;
        self.id[s] = self.count as i32;
        for w in g.adj(s as i32) {
            if !self.marked[*w as usize] {
                self.dfs(g, *w as usize);
            }
        } 
    }

    /// v 所在的连通分量的标识符。
    pub fn id(&self, v: i32) -> i32 {
        self.id[v as usize]
    }

    /// v 和 w 是否连通。
    pub fn connected(&self, v: i32, w: i32) -> bool {
        self.id[v as usize] == self.id[w as usize]
    }

    /// 连通分量数。
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph_data;
    use super::*;

    #[test]
    fn connected_test() {
        let g = graph_data();
        let d = CC::new(g);
        assert!(d.connected(0, 1));
    }

    #[test]
    fn count_test() {
        let g = graph_data();
        let d = CC::new(g);
        assert_eq!(1, d.count());
    }

    #[test]
    fn id_test() {
        let g = graph_data();
        let d = CC::new(g);
        assert_eq!(0, d.id(1));
    }
}