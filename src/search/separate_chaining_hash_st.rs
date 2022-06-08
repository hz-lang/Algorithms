//! 基于拉链法的哈希表。

use super::sequential_search::SequentialSearchST;

struct SeparateChainingHashST {
    n: usize, // 键值对总数。
    m: usize, // 散列表的大小。
    st: Vec<SequentialSearchST>,
}

impl SeparateChainingHashST {
    /// 创建一个空的散列表。
    pub fn new() -> Self {
        Self {
            n: 0,
            m: 997,
            st: vec![],
        }
    }

    /// 根据键，查找值。
    pub fn get(&self, key: i32) -> Option<i32> {
        let i = self.hash(key);
        self.st[i].get(key)
    }

    /// 插入键值对。
    pub fn put(&mut self, key: i32, value: i32) {
        let i = self.hash(key);
        self.resize(i);

        self.n += 1;
        self.st[i].put(key, value);
    }

    fn resize(&mut self, i: usize) {
        // 借助 Vec 自动调整缓存大小。
        while self.st.len() <= i {
            self.st.push(SequentialSearchST::new());
        }
    }

    /// 获取哈希值。
    fn hash(&self, key: i32) -> usize {
        (key & 0x7FFFFFFF) as usize % self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut h = SeparateChainingHashST::new();
        h.put(1, 99);
        assert_eq!(Some(99), h.get(1));
    }
}