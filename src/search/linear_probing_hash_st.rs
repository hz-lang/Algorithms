//! 线性探测符号表。

pub struct LinearProbingHashST {
    n: usize,               // 键值对的总数。
    m: usize,               // 符号表大小。
    keys: Vec<Option<i32>>, // 键。
    values: Vec<i32>,       // 值。
}

impl LinearProbingHashST {
    /// 创建一个空的符号表。
    pub fn new() -> Self {
        let m = 16;
        let keys = vec![None; m];
        let values = vec![0; m];
        Self {
            n: 0,
            m,
            keys,
            values,
        }
    }

    /// 插入键值对。
    pub fn put(&mut self, key: i32, value: i32) {
        if self.n >= self.m / 2 {
            self.resize(2 * self.m);
        }

        let mut i = self.hash(key);
        while let Some(k) = self.keys[i] {
            if k == key {
                self.values[i] = value;
                return;
            }
            i = (i + 1) % self.m;
        }

        self.keys[i] = Some(key);
        self.values[i] = value;
        self.n += 1;
    }

    /// 查找指定的键。
    pub fn get(&self, key: i32) -> Option<i32> {
        let mut i = self.hash(key);
        while let Some(k) = self.keys[i] {
            if k == key {
                return Some(self.values[i]);
            }
            i = (i + 1) % self.m;
        }
        None
    }

    /// 删除指定的键值对。
    pub fn delete(&mut self, key: i32) {
        if !self.contains(key) {
            return;
        }

        let mut i = self.hash(key);
        while self.keys[i].filter(|k| *k != key).is_some() {
            i = (i + 1) % self.m;
        }

        self.keys[i] = None;
        self.values[i] = 0;

        i = (i + 1) % self.m;
        while let Some(k) = self.keys[i] {
            let value = self.values[i];

            self.keys[i] = None;
            self.values[i] = 0;
            self.n -= 1;
            self.put(k, value);

            i = (i + 1) % self.m;
        }

        self.n -= 1;
        if self.n > 0 && self.n == self.m / 8 {
            self.resize(self.m / 2);
        }
    }

    fn hash(&self, key: i32) -> usize {
        (key & 0x7FFFFFFF) as usize % self.m
    }

    fn resize(&mut self, m: usize) {
        let mut t = LinearProbingHashST::with_capacity(m);
        for i in 0..m {
            if let Some(k) = self.keys[i] {
                t.put(k, self.values[i]);
            }
        }

        *self = t;
    }

    fn contains(&self, key: i32) -> bool {
        let i = self.hash(key);
        self.keys[i].is_some()
    }

    fn with_capacity(m: usize) -> Self {
        Self {
            n: 0,
            m,
            keys: vec![None; m],
            values: vec![0; m],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let t = create();

        assert_eq!(3, t.n);
        assert_eq!(Some(99), t.get(1));
        assert_eq!(Some(199), t.get(3));
        assert_eq!(Some(299), t.get(5));
    }

    #[test]
    fn delete_test() {
        let mut t = create();
        t.delete(1);

        assert_eq!(2, t.n);
        assert_eq!(None, t.get(1));
        assert_eq!(Some(199), t.get(3));
        assert_eq!(Some(299), t.get(5));
    }

    fn create() -> LinearProbingHashST {
        let mut t = LinearProbingHashST::new();
        t.put(1, 99);
        t.put(3, 199);
        t.put(5, 299);
        t
    }
}
