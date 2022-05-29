//! 二叉堆。

struct BinaryHeap {
    buf: Vec<i32>,
    len: usize,
}

impl BinaryHeap {
    /// 创建一个指定容量的二叉堆。
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity + 1],
            len: 0, // 数组的第一个元素没有使用。
        }
    }

    /// 获取二叉堆的迭代器。
    pub fn iter(&self) -> BinaryHeapIter {
        BinaryHeapIter {
            i: 0,
            buf: &self.buf[1..=self.len],
        }
    }

    /// 获取二叉堆的元素数。
    fn len(&self) -> usize {
        self.len
    }

    /// 在二叉堆最后添加一个元素。
    fn push(&mut self, item: i32) {
        self.len += 1;
        self.buf[self.len] = item;
    }

    /// 从二叉堆顶部取出一个元素，并用最后一个元素顶替。
    fn pop(&mut self) -> i32 {
        let max = self.buf[1];
        self.buf[1] = i32::MIN; // 清除最大元素。
        self.buf.swap(1, self.len);
        self.len -= 1;
        max
    }

    /// 上浮。
    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.buf[k / 2] < self.buf[k] {
            self.buf.swap(k / 2, k);
            k /= 2;
        }
    }

    /// 下沉。
    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.len {
            let mut j = 2 * k;
            if j < self.len && self.buf[j] < self.buf[j + 1] {
                j += 1;
            }
            if self.buf[k] >= self.buf[j] {
                break;
            }
            self.buf.swap(k, j);
            k = j;
        }
    }
}

/// 二叉堆的迭代器。
struct BinaryHeapIter<'a> {
    i: usize,
    buf: &'a [i32],
}

impl<'a> Iterator for BinaryHeapIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i < self.buf.len() {
            self.i += 1;
            Some(&self.buf[i])
        } else {
            None
        }
    }
}

/// 优先队列。
struct MaxPQ {
    h: BinaryHeap,
}

impl MaxPQ {
    /// 创建一个指定大小的优先队列。
    fn new(capacity: usize) -> Self {
        Self {
            h: BinaryHeap::new(capacity),
        }
    }

    /// 插入新元素。
    fn insert(&mut self, item: i32) {
        self.h.push(item);
        self.h.swim(self.h.len());
    }

    /// 删除并返回最大元素。
    fn delete_max(&mut self) -> i32 {
        let max = self.h.pop();
        self.h.swim(1);
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::binary_heap::MaxPQ;

    #[test]
    fn insert() {
        let mut q = MaxPQ::new(4);
        q.insert(2);
        q.insert(4);
        q.insert(8);

        let mut iter = q.h.iter();
        assert_eq!(Some(&8), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn delete_max() {
        let mut q = MaxPQ::new(4);
        q.insert(2);
        q.insert(4);
        q.insert(8);

        let v = q.delete_max();
        assert_eq!(8, v);

        let mut iter = q.h.iter();
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
    }
}
