//! 符号表。

use super::Graph;
use std::{collections::HashMap, slice, str};

pub struct SymbolGraph {
    st: HashMap<String, usize>,    // 符号-索引。
    keys: Vec<(*const u8, usize)>, // 索引-符号（符号由指针和字符串长度构成）。
    g: Graph,                      // 图。
}

impl SymbolGraph {
    /// 创建一个新的 [`SymbolGraph`]。
    /// 参数 list 由字符串构成，字符串以空格分隔不同符号。
    pub fn new(list: &[&str]) -> Self {
        let mut s = Self {
            st: HashMap::with_capacity(list.len()),
            keys: Vec::with_capacity(list.len()),
            g: Graph::new(list.len()),
        };
        for &v in list {
            for k in v.split_ascii_whitespace() {
                if !s.contains(k) {
                    let m = k.to_owned();
                    s.keys.push((m.as_ptr(), m.len()));
                    s.st.insert(m, s.st.len());
                }
            }

            // 取第一个符号索引作为顶点。
            let mut a = v.split_ascii_whitespace();
            let v = a.next().map(|k| s.st[k]).expect("顶点之间必须有空格");
            for k in a {
                let w = s.st[k];
                s.g.add_edge(v as i32, w as i32);
            }
        }
        s
    }

    /// 是否包含符号。
    pub fn contains(&self, s: &str) -> bool {
        self.st.contains_key(s)
    }

    /// 获取符号对应的索引。
    pub fn index(&self, s: &str) -> usize {
        self.st[s]
    }

    /// 获取索引对应的符号。
    pub fn name(&self, v: usize) -> &str {
        unsafe {
            let (ptr, len) = self.keys[v];
            let slice = slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// 获取图。
    pub fn graph(&self) -> &Graph {
        &self.g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_test() {
        let s = create();
        assert!(s.contains("a"));
    }

    #[test]
    fn index_test() {
        let s = create();
        assert_eq!(0, s.index("a"));
    }

    #[test]
    fn name_test() {
        let s = create();
        assert_eq!("a", s.name(0));
    }

    fn create() -> SymbolGraph {
        let list = ["a b", "b c", "c a"];
        let s = SymbolGraph::new(&list[..]);
        println!("图：{:?}", s.graph());

        s
    }
}
