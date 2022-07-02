//! 加权有向边。

use std::fmt::{Display, Formatter, self};

pub struct DirectedEdge {
    v: usize,    // 边的起点。&
    w: usize,    // 边的终点。
    weight: f64, // 边的权重。
}

impl DirectedEdge {
    /// 创建 [`DirectedEdge`] 的新实例。
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Self { v, w, weight }
    }

    /// 获取边的权重。
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// 获取边的起点。
    pub fn from(&self) -> usize {
        self.v
    }

    /// 获取边的终点。
    pub fn to(&self) -> usize {
        self.w
    }
}

/// 对象的字符串形式。
impl Display for DirectedEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}->{} {:.2}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let d = DirectedEdge::new(1, 2, 5.6);
        assert_eq!("1->2 5.60\n", &format!("{d}"));
    }
}