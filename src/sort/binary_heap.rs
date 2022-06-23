//! 二叉堆。

use std::slice::Iter;

pub struct BinaryHeap<T> {
    buf: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: Clone + PartialOrd + PartialEq + Default,
{
    /// 创建一个指定容量的二叉堆。
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// 利用[`Vec`]创建一个二叉堆。
    pub fn from_vec(buf: Vec<T>) -> Self {
        Self { buf }
    }

    /// 获取二叉堆的迭代器。
    pub fn iter(&self) -> BinaryHeapIter<T> {
        BinaryHeapIter {
            it: self.buf.iter(),
        }
    }

    /// 获取二叉堆的元素数。
    fn len(&self) -> usize {
        self.buf.len()
    }

    /// 在二叉堆最后添加一个元素。
    fn push(&mut self, item: T) {
        self.buf.push(item);
    }

    /// 从二叉堆顶部取出一个元素，并用最后一个元素顶替。
    fn pop(&mut self) -> T {
        let max = self.buf[0].clone();
        let len = self.len() - 1;
        self.buf.swap(0, len);
        self.buf.remove(len);
        max
    }

    /// 上浮。
    pub fn swim(&mut self, k: usize) {
        swim(&mut self.buf, k);
    }

    /// 下沉。
    pub fn sink(&mut self, k: usize) {
        sink(&mut self.buf, k)
    }

    /// 获取二叉堆的切片。
    pub fn as_slice(&self) -> &[T] {
        &self.buf
    }

    /// 获取二叉堆的可变切片。
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.buf
    }
}

/// 获取左叶子。
fn left(k: usize) -> usize {
    (k << 1) + 1
}

/// 获取右叶子。
fn right(k: usize) -> usize {
    (k + 1) << 1
}

/// 获取父节点。
fn parent(k: usize) -> usize {
    match k {
        0 => 0,
        _ => (k - 1) >> 1,
    }
}

/// 上浮。
fn swim<T>(a: &mut [T], mut k: usize)
where
    T: PartialOrd,
{
    let mut i = parent(k);
    while k > 0 && a[i] < a[k] {
        a.swap(i, k);
        k = i;
        i = parent(k);
    }
}

/// 下沉。
fn sink<T>(a: &mut [T], mut k: usize)
where
    T: PartialOrd,
{
    let mut j = left(k);
    while j < a.len() {
        if j + 1 < a.len() && a[j] < a[j + 1] {
            j += 1;
        }
        if a[k] >= a[j] {
            break;
        }
        a.swap(k, j);
        k = j;
        j = left(k);
    }
}

/// 二叉堆的迭代器。
pub struct BinaryHeapIter<'a, T> {
    it: Iter<'a, T>,
}

impl<'a, T> Iterator for BinaryHeapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

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
    use crate::sort::{
        binary_heap::{HeapSort, MaxPQ},
        is_sorted,
    };

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

    #[test]
    fn sort() {
        let a = vec![8, 5, 7, 6, 4, 9, 3, 1];
        let mut bs = HeapSort::new(a);
        assert_eq!(&[9, 6, 8, 5, 4, 7, 3, 1], &bs.buf.buf[..]);

        bs.sort();
        assert!(is_sorted(&bs.buf.buf[..]));
    }
}
