//! 计算强连通分量的 Kosaraju 算法。

use super::{depth_first_order::DepthFirstOrder, digraph::Digraph};

pub struct KosarajuSCC {
    marked: Vec<bool>, // 已访问过的顶点。
    id: Vec<usize>,    // 强连通分量的标识符。
    count: usize,      // 强连通分量的数量。
}

impl KosarajuSCC {
    pub fn new(g: &mut Digraph) -> Self {
        let g = &mut g.reverse();
        let mut k = Self {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            count: 0,
        };
        let order = DepthFirstOrder::new(g);
        for s in order.reverse_post() {
            if !k.marked[*s] {
                k.dfs(g, *s);
                k.count += 1;
            }
        }
        k
    }

    fn dfs(&mut self, g: &Digraph, s: usize) {
        self.marked[s] = true;
        self.id[s] = self.count;
        for w in g.adj(s) {
            if !self.marked[*w] {
                self.dfs(g, *w);
            }
        }
    }

    /// 是否是强连通。
    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    /// 强连通分量标识符。
    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    /// 强连通分量的总数。
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strongly_connected_test() {
        let k = create();
        assert!(k.strongly_connected(1, 2));
        assert!(k.strongly_connected(3, 2));
        assert!(k.strongly_connected(1, 3));
    }

    #[test]
    fn count_test() {
        let k = create();
        assert_eq!(2, k.count());
    }

    #[test]
    fn id_test() {
        let k = create();
        assert_eq!(0, k.id(1));
        assert_eq!(0, k.id(2));
        assert_eq!(0, k.id(3));
    }

    fn create() -> KosarajuSCC {
        let mut g = Digraph::new(4);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
        KosarajuSCC::new(&mut g)
    }
}
