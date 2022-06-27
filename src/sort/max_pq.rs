use super::binary_heap::BinaryHeap;

/// 优先队列。
pub struct MaxPQ<T> {
    h: BinaryHeap<T>,
}

impl<T> MaxPQ<T>
where
    T: Default + Clone + PartialEq + PartialOrd,
{
    /// 创建一个指定大小的优先队列。
    fn new(capacity: usize) -> Self {
        Self {
            h: BinaryHeap::new(capacity),
        }
    }

    /// 插入新元素。
    fn insert(&mut self, item: T) {
        self.h.push(item);
        self.h.swim(self.h.len() - 1);
    }

    /// 删除并返回最大元素。
    fn delete_max(&mut self) -> T {
        let max = self.h.pop();
        self.h.swim(0);
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::max_pq::MaxPQ;

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
