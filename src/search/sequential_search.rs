//! 顺序查找。

use std::{fmt::Display, marker::PhantomData, ptr::NonNull};

/// 基于无序链表。
struct SequentialSearchST {
    head: Option<NonNull<Node>>,
    marker: PhantomData<Box<Node>>,
}

struct Node {
    key: i32,
    value: Option<i32>,
    next: Option<NonNull<Node>>,
}

impl Node {
    fn new(key: i32, value: Option<i32>, next: Option<NonNull<Node>>) -> Self {
        Self { key, value, next }
    }
}

impl Display for SequentialSearchST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let mut x = self.head.as_ref();
            while let Some(node) = x {
                let p = node.as_ptr();
                write!(f, "\n({}, {:?})", (*p).key, (*p).value)?;
                x = (*p).next.as_ref();
            }
            writeln!(f)
        }
    }
}

impl Drop for SequentialSearchST {
    fn drop(&mut self) {
        unsafe {
            while let Some(node) = self.head.take() {
                let mut n = Box::from_raw(node.as_ptr());
                self.head = n.next.take();
            }
        }
    }
}

impl SequentialSearchST {
    // 创建一个新的无序链表。
    fn new() -> Self {
        Self {
            head: None,
            marker: PhantomData,
        }
    }

    // 查找指定的值。
    fn get(&self, key: i32) -> Option<i32> {
        unsafe {
            let mut x = self.head.as_ref();
            while let Some(node) = x {
                let p = node.as_ptr();
                debug_assert!(!p.is_null());
                if key == (*p).key {
                    // 命中，返回。
                    return (*p).value;
                }
                x = (*p).next.as_ref();
            }
        }
        None // 未命中。
    }

    // 添加一个键值对。
    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            let mut x = self.head.as_mut();
            while let Some(node) = x {
                let p = node.as_ptr();
                debug_assert!(!p.is_null());
                if key == (*p).key {
                    // 命中，更新。
                    (*p).value = Some(value);
                    return;
                }
                x = (*p).next.as_mut();
            }

            // 未命中，插入根节点前面。
            let node = Node::new(key, Some(value), self.head);
            self.head = Some(Box::leak(Box::new(node)).into())
        }
    }

    // 删除指定的键值对。
    fn delete(&mut self, key: i32) {
        unsafe {
            let mut previous = self.head.as_ref();
            let mut x = previous.map(|n| (*n.as_ptr()).next.as_ref()).flatten();
            while let Some(node) = x {
                let p = node.as_ptr();
                debug_assert!(!p.is_null());
                if key == (*p).key {
                    // 命中，更新。
                    previous.map(|n| {
                        (*n.as_ptr()).next = (*p).next;
                    });
                    // 释放命中节点。
                    Box::from_raw(p);
                    return;
                }
                previous = x;
                x = (*p).next.as_ref();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut st = SequentialSearchST::new();
        st.put(1, 5);
        st.put(12, 55);
        println!("st:{st}");

        assert_eq!(Some(55), st.get(12));
        assert_eq!(Some(5), st.get(1));

        st.delete(1);
        assert_eq!(None, st.get(1));

        st.put(12, 33);
        assert_eq!(Some(33), st.get(12));
    }
}
