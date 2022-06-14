//! 有向图。

#[derive(Debug)]
pub struct Digraph {
    v: usize,             // 顶点数。
    e: usize,             // 边的数目。
    adj: Vec<Vec<usize>>, // 邻接表。
}

impl Digraph {
    // 创建一个空图。
    pub fn new(v: usize) -> Self {
        Self {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }

    /// 添加一条边。
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.e += 1;
    }

    /// 获取邻接的顶点。
    pub fn adj(&self, v: usize) -> &[usize] {
        &self.adj[v][..]
    }

    /// 获取图的顶点数。
    pub fn v(&self) -> usize {
        self.v
    }

    /// 获取图的边数。
    pub fn e(&self) -> usize {
        self.e
    }

    /// 创建一个反向的图。
    pub fn reverse(&self) -> Self {
        let mut r = Self::new(self.v);
        for v in 0..self.v {
            for w in self.adj(v) {
                r.add_edge(*w, v);
            }
        }
        r
    }
}
