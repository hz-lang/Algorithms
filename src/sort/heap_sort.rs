use super::binary_heap::{sink, BinaryHeap};

/// 堆排序。
pub struct HeapSort<T> {
    buf: BinaryHeap<T>,
}

impl<T> HeapSort<T>
where
    T: Default + Clone + PartialEq + PartialOrd,
{
    /// 创建一个二叉堆。
    pub fn new(a: Vec<T>) -> Self {
        let mut h = Self {
            buf: BinaryHeap::from_vec(a),
        };
        h.build_max_heap();
        h
    }

    /// 排序。
    pub fn sort(&mut self) {
        let a = self.buf.as_mut_slice();
        // 循环时不要包含索引 0。
        for i in (1..a.len()).rev() {
            a.swap(0, i);
            // 下沉时，不断排除后面已排序的部分。
            sink(&mut a[..i], 0);
        }
    }

    /// 构造最大堆。
    fn build_max_heap(&mut self) {
        let len = self.buf.len() / 2;
        for i in (0..len).rev() {
            self.buf.sink(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{heap_sort::HeapSort, is_sorted};

    #[test]
    fn sort() {
        let a = vec![8, 5, 7, 6, 4, 9, 3, 1];
        let mut bs = HeapSort::new(a);
        assert_eq!(&[9, 6, 8, 5, 4, 7, 3, 1], bs.buf.as_slice());

        bs.sort();
        assert!(is_sorted(bs.buf.as_slice()));
    }
}
