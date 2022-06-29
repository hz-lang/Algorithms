use super::binary_heap::*;

/// 最小堆。
pub struct MinPQ<T> {
    h: Vec<T>,
}

impl<T> MinPQ<T>
where
    T: PartialEq + PartialOrd,
{
    /// 创建一个指定大小的最小堆。
    pub fn new(capacity: usize) -> Self {
        Self {
            h: Vec::with_capacity(capacity),
        }
    }

    /// 插入新元素。
    pub fn insert(&mut self, item: T) {
        self.h.push(item);
        let i = self.h.len() - 1;
        swim(&mut self.h, i);
    }

    /// 删除并返回最小元素。
    pub fn delete_min(&mut self) -> Option<T> {
        let i = self.h.len() - 1;
        self.h.swap(0, i);

        let min = self.h.pop();
        sink(&mut self.h, 0);
        min
    }

    /// 是否符合指定的条件。
    pub fn get_mut<F>(&mut self, f: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool,
    {
        for v in self.h.iter_mut() {
            if f(v) {
                return Some(v);
            }
        }
        None
    }
}

/// 小鱼上浮。
fn swim<T>(a: &mut [T], i: usize)
where
    T: PartialOrd,
{
    if i == 0 {
        return;
    }

    let parent = parent(i);
    if a[parent] > a[i] {
        a.swap(parent, i);
        swim(a, parent);
    }
}

/// 大鱼下沉。
fn sink<T>(a: &mut [T], i: usize)
where
    T: PartialOrd,
{
    let l = left(i);
    if l >= a.len() {
        return;
    }

    if a[l] < a[i] {
        a.swap(l, i);
        sink(a, l);
    }

    let r = right(i);
    if r >= a.len() {
        return;
    }

    if a[r] < a[i] {
        a.swap(r, i);
        sink(a, r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut q = MinPQ::new(4);
        q.insert(8);
        q.insert(4);
        q.insert(2);

        assert_eq!(&[2, 8, 4], &q.h[..]);
    }

    #[test]
    fn delete_max() {
        let mut q = MinPQ::new(4);
        q.insert(8);
        q.insert(4);
        q.insert(2);

        let v = q.delete_min();
        assert_eq!(Some(2), v);
        assert_eq!(&[4, 8], &q.h[..]);
    }
}
