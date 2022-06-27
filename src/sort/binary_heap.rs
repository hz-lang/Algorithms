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
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// 在二叉堆最后添加一个元素。
    pub fn push(&mut self, item: T) {
        self.buf.push(item);
    }

    /// 从二叉堆顶部取出一个元素，并用最后一个元素顶替。
    pub fn pop(&mut self) -> T {
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
pub(crate) fn sink<T>(a: &mut [T], mut k: usize)
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
