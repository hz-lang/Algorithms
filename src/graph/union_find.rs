//! 并查集。

pub struct UF {
    id: Vec<usize>, // 父链接数组（由触点索引）。
    sz: Vec<usize>, // 由触点索引的根节点对应的分量大小。
    count: usize,
}

impl UF {
    /// 创建一个新的 [`UF`]。
    pub fn new(n: usize) -> Self {
        let mut id = Vec::with_capacity(n);
        let mut sz = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
            sz.push(1);
        }
        Self { id, sz, count: n }
    }

    /// 连通分量的数量。
    pub fn count(&self) -> usize {
        self.count
    }

    /// p 所在的分量标志符。
    pub fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    /// p 和 q 是否在同一个分量中。
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// 连接 p 和 q。
    pub fn union(&mut self, p: usize, q: usize) {
        let (i, j) = (self.find(p), self.find(q));
        if i == j {
            return;
        }

        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }

        self.count -= 1;
    }
}
