//! 二分查找。

struct BinarySearchST {
    keys: Vec<i32>,
    values: Vec<i32>,
    len: usize,
}

impl BinarySearchST {
    /// 创建一个新的二分查找符号表。
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            len: 0,
        }
    }

    /// 获取指定的键值。
    pub fn get(&self, key: i32) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        let i = self.rank(key);
        if !self.contains_key(i, key) {
            return None;
        }

        Some(self.values[i])
    }

    /// 添加一对键值。
    pub fn put(&mut self, key: i32, value: i32) {
        let i = self.rank(key);
        if self.contains_key(i, key) {
            self.values[i] = value;
            return;
        }

        // 调整长度。
        self.resize();

        for k in (i + 1..=self.len).rev() {
            self.keys[k] = self.keys[k - 1];
            self.values[k] = self.values[k - 1];
        }
        self.keys[i] = key;
        self.values[i] = value;
        self.len += 1;
    }

    /// 删除指定的键值对。
    pub fn delete(&mut self, key: i32) {
        let i = self.rank(key);
        if !self.contains_key(i, key) {
            return;
        }

        for k in i + 1..self.len {
            self.keys[k - 1] = self.keys[k];
            self.values[k - 1] = self.values[k];
        }
        self.len -= 1;
    }

    fn contains_key(&self, i: usize, key: i32) -> bool {
        i < self.len && key == self.keys[i]
    }

    fn resize(&mut self) {
        if self.len < self.keys.len() {
            return;
        }
        self.keys.push(0);
        self.values.push(0);
    }

    fn rank(&self, key: i32) -> usize {
        let (mut lo, mut hi) = (0, self.len as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if self.keys[mid as usize] == key {
                return mid as usize;
            }
            if self.keys[mid as usize] < key {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        lo as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bs = BinarySearchST::new();
        bs.put(5, 105);
        bs.put(9, 350);
        bs.put(2, 999);

        assert_eq!(3, bs.len);
        assert_eq!(Some(999), bs.get(2));
        assert_eq!(Some(350), bs.get(9));
        assert_eq!(Some(105), bs.get(5));

        bs.delete(2);
        assert_eq!(2, bs.len);
        assert_eq!(None, bs.get(2));
    }
}
