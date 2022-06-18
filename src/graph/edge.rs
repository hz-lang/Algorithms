//! 带权重的边。

use std::{
    cmp,
    fmt::{self, Display},
};

pub struct Edge {
    v: usize,    // 边的顶点。
    w: usize,    // 边的另一个顶点。
    weight: f64, // 边的权重。
}

impl Edge {
    /// 获取一个新的 [`Edge`]。
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Self { v, w, weight }
    }

    /// 获取边的权重。
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// 获取边的顶点。
    pub fn either(&self) -> usize {
        self.v
    }

    /// 获取指定顶点的另一个顶点。
    pub fn other(&self, vertex: usize) -> Option<usize> {
        if vertex == self.v {
            Some(self.w)
        } else if vertex == self.w {
            Some(self.v)
        } else {
            None
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} {:.2}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn other_test() {
        let e = create();
        assert_eq!(Some(3), e.other(2));
    }

    #[test]
    fn fmt_test() {
        let e = create();
        assert_eq!("2-3 0.85", format!("{}", e));
    }

    fn create() -> Edge {
        Edge::new(2, 3, 0.85)
    }
}
