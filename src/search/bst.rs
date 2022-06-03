//! 基于二叉查找树的符号表。

use std::{collections::VecDeque, fmt::Display, marker::PhantomData, ptr::NonNull};

type Link = Option<NonNull<Node>>;

pub struct BST {
    root: Link,
    marker: PhantomData<Box<Node>>,
}

impl BST {
    /// 创建一个空的符号表。
    pub fn new() -> Self {
        Self {
            root: None,
            marker: PhantomData,
        }
    }

    /// 获取节点数。
    pub fn size(&self) -> usize {
        self.size_of(self.root)
    }

    fn size_of(&self, node: Link) -> usize {
        node.map_or(0, |n| unsafe { (*n.as_ptr()).len })
    }

    /// 获取指定键对应的值。
    pub fn get(&self, key: i32) -> Option<i32> {
        self.get_from(self.root, key)
    }

    fn get_from(&self, root: Link, key: i32) -> Option<i32> {
        root.map_or(None, |node| unsafe {
            let n = node.as_ptr();
            if (*n).key == key {
                Some((*n).value)
            } else if (*n).key < key {
                self.get_from((*n).right, key)
            } else {
                self.get_from((*n).left, key)
            }
        })
    }

    /// 插入键值对。
    pub fn put(&mut self, key: i32, value: i32) {
        self.root = self.put_with(self.root, key, value);
    }

    fn put_with(&self, root: Link, key: i32, value: i32) -> Link {
        match root {
            None => {
                let n = Node::new(key, value, 1);
                Some(Box::leak(Box::new(n)).into())
            }
            Some(node) => unsafe {
                let mut n = node.as_ptr();
                if key == (*n).key {
                    (*n).value = value;
                } else if key > (*n).key {
                    (*n).right = self.put_with((*n).right, key, value);
                } else {
                    (*n).left = self.put_with((*n).left, key, value);
                }
                (*n).len = self.size_of((*n).left) + self.size_of((*n).right) + 1;
                root
            },
        }
    }

    /// 获取最小键。
    pub fn min(&self) -> Option<i32> {
        self.min_node(self.root)
            .map(|n| unsafe { (*n.as_ptr()).key })
    }

    fn min_node(&self, root: Link) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            if (*node).left.is_none() {
                root
            } else {
                self.min_node((*node).left)
            }
        })
    }

    /// 获取最大的键。
    pub fn max(&self) -> Option<i32> {
        self.max_node(self.root)
            .map(|n| unsafe { (*n.as_ptr()).key })
    }

    fn max_node(&self, root: Link) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            if (*node).right.is_none() {
                root
            } else {
                self.max_node((*node).right)
            }
        })
    }

    /// 向下取整。
    pub fn floor(&self, key: i32) -> Option<i32> {
        self.floor_node(self.root, key)
            .map(|n| unsafe { (*n.as_ptr()).key })
    }

    fn floor_node(&self, root: Link, key: i32) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            if key == (*node).key {
                root
            } else if key < (*node).key {
                self.floor_node((*node).left, key)
            } else {
                self.floor_node((*node).right, key).or(root)
            }
        })
    }

    /// 选择排名为 k 的键。
    pub fn select(&self, k: usize) -> Option<i32> {
        self.select_node(self.root, k)
            .map(|n| unsafe { (*n.as_ptr()).key })
    }

    fn select_node(&self, root: Link, k: usize) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            let t = self.size_of((*node).left);
            if t == k {
                root
            } else if t > k {
                self.select_node((*node).left, k)
            } else {
                self.select_node((*node).right, k - t - 1)
            }
        })
    }

    /// 获取指定键的排名。
    pub fn rank(&self, key: i32) -> usize {
        self.rank_from(self.root, key)
    }

    fn rank_from(&self, root: Link, key: i32) -> usize {
        root.map_or(0, |n| unsafe {
            let node = n.as_ptr();
            if key == (*node).key {
                self.size_of((*node).left)
            } else if key < (*node).key {
                self.rank_from((*node).left, key)
            } else {
                1 + self.size_of((*node).left) + self.rank_from((*node).right, key)
            }
        })
    }

    /// 删除最小节点。
    pub fn delete_min(&mut self) {
        self.root = self.delete_min_node(self.root);
    }

    fn delete_min_node(&self, root: Link) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            if (*node).left.is_none() {
                // 回收节点内存。
                Box::from_raw(node).right
            } else {
                (*node).left = self.delete_min_node((*node).left);
                (*node).len = self.size_of((*node).left) + self.size_of((*node).right) + 1;
                root
            }
        })
    }

    /// 删除指定节点。
    pub fn delete(&mut self, key: i32) {
        self.root = self.delete_node(self.root, key);
    }

    fn delete_node(&self, mut root: Link, key: i32) -> Link {
        root.map_or(None, |n| unsafe {
            let node = n.as_ptr();
            if key < (*node).key {
                (*node).left = self.delete_node((*node).left, key);
            } else if key > (*node).key {
                (*node).right = self.delete_node((*node).right, key);
            } else {
                if (*node).left.is_none() {
                    return (*node).right;
                }
                if (*node).right.is_none() {
                    return (*node).left;
                }
                // 回收节点内存。
                let t = Box::from_raw(node);
                root = self.min_node(t.right);
                root.map(|x| {
                    (*x.as_ptr()).right = self.delete_min_node(t.right);
                    (*x.as_ptr()).left = t.left;
                });
            }
            (*node).len = self.size_of((*node).left) + self.size_of((*node).right) + 1;
            root
        })
    }

    /// 获取所有的键。
    pub fn keys(&self) -> VecDeque<i32> {
        self.min()
            .zip(self.max())
            .map_or_else(|| VecDeque::new(), |scope| self.keys_from(scope))
    }

    /// 获取指定范围的键。
    pub fn keys_from(&self, scope: (i32, i32)) -> VecDeque<i32> {
        let mut q = VecDeque::new();
        self.keys_from_impl(self.root, &mut q, scope);
        q
    }

    fn keys_from_impl(&self, root: Link, q: &mut VecDeque<i32>, scope: (i32, i32)) {
        root.map(|n| unsafe {
            let x = n.as_ptr();
            let (lo, hi) = scope;
            if (*x).key > lo {
                self.keys_from_impl((*x).left, q, scope);
            }
            if (*x).key >= lo && (*x).key <= hi {
                q.push_front((*x).key);
            }
            if (*x).key < hi {
                self.keys_from_impl((*x).right, q, scope);
            }
        });
    }
}

impl Display for BST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe fn print(node: Link, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(n) = node {
                let x = n.as_ptr();
                print((*x).left, f)?;
                write!(f, "\n({}, {})", (*x).key, (*x).value)?;
                print((*x).right, f)
            } else {
                write!(f, "")
            }
        }
        unsafe { print(self.root, f) }
    }
}

impl Drop for BST {
    fn drop(&mut self) {
        fn drop_impl(node: Link) {
            node.map(|n| unsafe {
                // 释放节点内存。
                let x = Box::from_raw(n.as_ptr());
                drop_impl(x.left);
                drop_impl(x.right);
            });
        }
        drop_impl(self.root);
    }
}

struct Node {
    key: i32,
    value: i32,
    len: usize,
    left: Link,
    right: Link,
}

impl Node {
    fn new(key: i32, value: i32, len: usize) -> Self {
        Self {
            key,
            value,
            len,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_test() {
        let mut bst = create_bst();

        assert_eq!(3, bst.size());
        assert_eq!(Some(42), bst.get(1));
        assert_eq!(Some(98), bst.get(2));
        assert_eq!(Some(75), bst.get(5));

        bst.put(2, 99);
        assert_eq!(Some(99), bst.get(2));
    }

    fn create_bst() -> BST {
        let mut bst = BST::new();
        bst.put(2, 98);
        bst.put(1, 42);
        bst.put(5, 75);
        println!("bst:{}", &bst);
        bst
    }

    #[test]
    fn min_key_test() {
        let bst = create_bst();
        assert_eq!(Some(1), bst.min());
    }

    #[test]
    fn max_key_test() {
        let bst = create_bst();
        assert_eq!(Some(5), bst.max());
    }

    #[test]
    fn floor_test() {
        let bst = create_bst();
        assert_eq!(Some(2), bst.floor(3));
    }

    #[test]
    fn select_test() {
        let bst = create_bst();
        assert_eq!(Some(1), bst.select(0));
    }

    #[test]
    fn rank_test() {
        let bst = create_bst();
        assert_eq!(0, bst.rank(1));
    }

    #[test]
    fn delete_min_test() {
        let mut bst = create_bst();
        assert_eq!(Some(42), bst.get(1));

        bst.delete_min();
        assert_eq!(None, bst.get(1));
        assert_eq!(Some(98), bst.get(2));
        assert_eq!(Some(75), bst.get(5));
    }

    #[test]
    fn delete_test() {
        let mut bst = create_bst();
        assert_eq!(Some(42), bst.get(1));

        bst.delete(1);
        assert_eq!(None, bst.get(1));
        assert_eq!(Some(98), bst.get(2));
        assert_eq!(Some(75), bst.get(5));
    }

    #[test]
    fn keys_test() {
        let bst = create_bst();
        let mut keys = bst.keys();
        assert_eq!(3, keys.len());
        assert_eq!(Some(1), keys.pop_back());
        assert_eq!(Some(2), keys.pop_back());
        assert_eq!(Some(5), keys.pop_back());
    }

    #[test]
    fn keys_from_test() {
        let bst = create_bst();
        let mut keys = bst.keys_from((2, 6));
        assert_eq!(2, keys.len());
        assert_eq!(Some(2), keys.pop_back());
        assert_eq!(Some(5), keys.pop_back());
    }
}
