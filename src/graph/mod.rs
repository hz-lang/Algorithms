//! 图。

mod breadth_first_paths;
mod cc;
mod depth_first_search;
mod cycle;
mod two_color;

pub struct Graph {
    v: usize,           // 顶点数。
    e: usize,           // 边的数目。
    adj: Vec<Vec<i32>>, // 邻接表。
}

impl Graph {
    // 创建一个空图。
    pub fn new(v: usize) -> Self {
        Self {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }

    /// 添加一条边。
    pub fn add_edge(&mut self, v: i32, w: i32) {
        self.adj[v as usize].push(w);
        self.adj[w as usize].push(v);
        self.e += 1;
    }

    /// 获取邻接的顶点。
    pub fn adj(&self, v: i32) -> &[i32] {
        &self.adj[v as usize][..]
    }

    /// 获取图的顶点数。
    pub fn v(&self) -> usize {
        self.v
    }

    /// 获取图的边数。
    pub fn e(&self) -> usize {
        self.e
    }
}

// 0 ------ 2
// |\      /|\
// | \    / | \
// |    1   |  \
// 5 ------ 3 - 4
fn graph_data() -> Graph {
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
