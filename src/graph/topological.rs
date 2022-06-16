//! 拓扑排序。

use super::{depth_first_order::DepthFirstOrder, digraph::Digraph, directed_cycle::DirectedCycle};

pub struct Topological {
    order: Vec<usize>,
}

impl Topological {
    /// 创建一个空的拓扑排序结构。
    pub fn new(g: &mut Digraph) -> Self {
        let mut t = Self { order: vec![] };
        let d = DirectedCycle::new(g);
        if !d.has_cycle() {
            let dfs = DepthFirstOrder::new(g);
            let order = dfs.reverse_post();
            t.order = order.map(|&v| v).collect();
        }
        t
    }

    /// 获取拓扑有序的所有顶点。
    pub fn order(&self) -> &[usize] {
        &self.order
    }

    /// 是否是有向无环图。
    pub fn is_dag(&self) -> bool {
        !self.order.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dag_test() {
        let t = create();
        assert!(t.is_dag());
    }

    #[test]
    fn order_test() {
        let t = create();
        assert_eq!(&[1,2,3,4,0], t.order());
    }

    fn create() -> Topological {
        let mut g = Digraph::new(5);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        Topological::new(&mut g)
    }
}
