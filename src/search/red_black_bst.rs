//! 红黑树符号表。

use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    marker::PhantomData,
    ptr::NonNull,
};

type Link<K, V> = Option<NonNull<Node<K, V>>>;

pub struct RedBlackBST<K, V> {
    root: Link<K, V>,
    marker: PhantomData<Box<Node<K, V>>>,
}

impl<K, V> RedBlackBST<K, V>
where
    K: Copy + PartialOrd,
    V: Clone,
{
    /// 创建一个空的红黑树。
    pub fn new() -> Self {
        Self {
            root: None,
            marker: PhantomData,
        }
    }

    /// 插入键值对。
    pub fn put(&mut self, key: K, value: V) {
        self.root = put_with(self.root, key, value);
        set_color(self.root, Color::Black);
    }

    /// 删除最小节点。
    pub fn delete_min(&mut self) {
        self.set_parent_red_if_child_black();
        self.root = del_min_node(self.root);
        set_color(self.root, Color::Black);
    }

    /// 删除最大键。
    pub fn delete_max(&mut self) {
        self.set_parent_red_if_child_black();
        self.root = del_max_node(self.root);
        set_color(self.root, Color::Black);
    }

    /// 删除指定节点。
    pub fn delete(&mut self, key: K) {
        if !is_red(left_of(self.root)) && !is_red(right_of(self.root)) {
            set_color(self.root, Color::Red);
        }
        self.root = del_node(self.root, key);
        set_color(self.root, Color::Black);
    }

    /// 获取最小键。
    pub fn min(&self) -> Link<K, V> {
        min_node(self.root)
    }

    fn set_parent_red_if_child_black(&mut self) {
        let n = self.root;
        if !is_red(left_of(n)) && !is_red(right_of(n)) {
            set_color(n, Color::Black);
        }
    }

    /// 获取红黑树的迭代器。
    pub fn iter<'a>(&self) -> Iter<'a, K, V> {
        Iter::new(self.root)
    }
}

impl<K, V> Display for RedBlackBST<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_tree(self.root, f)
    }
}

fn print_tree<K, V>(root: Link<K, V>, f: &mut std::fmt::Formatter) -> std::fmt::Result
where
    K: Debug,
    V: Debug,
{
    root.map_or(Ok(()), |n| {
        print_tree(left(n), f)?;
        let h = unsafe { n.as_ref() };
        write!(f, "\n({:?}, {:?}, {:?})", &h.key, &h.value, &h.color)?;
        print_tree(right(n), f)
    })
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
    len: usize,
    color: Color,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V, len: usize, color: Color) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            len,
            color,
        }
    }

    fn update_size(&mut self) {
        self.len = 1 + size_of(self.left) + size_of(self.right);
    }
}

/// 迭代器。
pub struct Iter<'a, K, V> {
    root: Link<K, V>,
    list: VecDeque<&'a Node<K, V>>,
    marker: PhantomData<&'a Node<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub fn new(root: Link<K, V>) -> Self {
        let list = VecDeque::new();
        Self {
            root,
            list,
            marker: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.root {
            let n = unsafe { n.as_ref() };
            self.list.push_front(n);
            self.root = n.left;
        }
        let node = self.list.pop_front();
        self.root = node.map_or(None, |n| n.right);
        node
    }
}

fn left<K, V>(h: NonNull<Node<K, V>>) -> Link<K, V> {
    unsafe { h.as_ref().left }
}

fn right<K, V>(h: NonNull<Node<K, V>>) -> Link<K, V> {
    unsafe { h.as_ref().right }
}

fn left_of<K, V>(h: Link<K, V>) -> Link<K, V> {
    h.map_or(None, |h| left(h))
}

fn right_of<K, V>(h: Link<K, V>) -> Link<K, V> {
    h.map_or(None, |h| right(h))
}

/// 查询指定的键。
fn get_from_node<K, V>(root: Link<K, V>, key: K) -> Link<K, V>
where
    K: PartialOrd,
{
    root.map_or(None, |n| {
        let n = unsafe { n.as_ref() };
        if key == n.key {
            root
        } else if key < n.key {
            get_from_node(n.left, key)
        } else {
            get_from_node(n.right, key)
        }
    })
}

fn get_from<'a, K, V>(root: Link<K, V>, key: K) -> Option<&'a Node<K, V>>
where
    K: PartialOrd,
{
    get_from_node(root, key).map(|n| unsafe { n.as_ref() })
}

fn min_node<K, V>(root: Link<K, V>) -> Link<K, V> {
    root.map_or(None, |n| unsafe {
        let n = n.as_ref();
        if n.left.is_none() {
            root
        } else {
            min_node(n.left)
        }
    })
}

fn del_node<K, V>(mut h: Link<K, V>, key: K) -> Link<K, V>
where
    K: Copy + PartialOrd,
    V: Clone,
{
    if let Some(n) = h {
        let n = unsafe { n.as_ref() };
        if key < n.key {
            if !is_red(n.left) && !is_red(left_of(n.left)) {
                h = move_red_left(h);
            }

            h.map(|mut n| unsafe {
                n.as_mut().left = del_node(left(n), key);
            });
        } else {
            if is_red(left_of(h)) {
                h = rotate_right(h);
            }

            if let Some(n) = h {
                let n = unsafe { n.as_ref() };
                if key == n.key && n.right.is_none() {
                    return None;
                }
            }

            if !is_red(right_of(h)) && !is_red(left_of(right_of(h))) {
                h = move_red_right(h);
            }

            if let Some(mut n) = h {
                let n = unsafe { n.as_mut() };
                if n.key != key {
                    n.right = del_node(n.right, key);
                } else {
                    min_node(n.right).map(|m| {
                        let m = unsafe { m.as_ref() };
                        get_from(n.right, m.key).map(|k| n.value = k.value.clone());
                        n.key = m.key;
                        n.right = del_min_node(n.right);
                    });
                }
            }
        }
    }

    balance(h)
}

/// 删除最大节点。
fn del_max_node<K, V>(mut h: Link<K, V>) -> Link<K, V> {
    if is_red(left_of(h)) {
        h = rotate_right(h);
    }

    if let Some(n) = h {
        if right(n).is_none() {
            // 释放节点内存。
            unsafe { Box::from_raw(n.as_ptr()) };
            return None;
        }

        if !is_red(right(n)) && !is_red(left_of(right(n))) {
            h = move_red_right(h);
        }
    }

    // 节点 h 可能已经改变。
    h.map(|mut n| unsafe {
        n.as_mut().right = del_max_node(right(n));
    });

    balance(h)
}

/// 假定 node 为红，其右子、右左子皆黑，将右子、右左子之一变红。
fn move_red_right<K, V>(mut h: Link<K, V>) -> Link<K, V> {
    flip_colors_for_del(h);

    if !is_red(left_of(left_of(h))) {
        h = rotate_right(h);
    }

    h
}

/// 删除最小节点。
fn del_min_node<K, V>(mut h: Link<K, V>) -> Link<K, V> {
    if let Some(n) = h {
        if left(n).is_none() {
            // 释放节点内存。
            unsafe { Box::from_raw(n.as_ptr()) };
            return None;
        }

        if !is_red(left(n)) && !is_red(left_of(left(n))) {
            h = move_red_left(h);
        }
    }

    h.map(|mut n| unsafe {
        n.as_mut().left = del_min_node(left(n));
    });

    balance(h)
}

fn balance<K, V>(mut h: Link<K, V>) -> Link<K, V> {
    if is_red(right_of(h)) {
        h = rotate_left(h);
    }

    if is_red(right_of(h)) && !is_red(left_of(h)) {
        h = rotate_left(h);
    }

    if is_red(left_of(h)) && is_red(left_of(left_of(h))) {
        h = rotate_right(h);
    }

    if is_red(left_of(h)) && is_red(right_of(h)) {
        flip_colors(h);
    }

    h.map(|mut n| unsafe {
        n.as_mut().update_size();
    });

    h
}

/// 假定 node 为红，其左子、左左子皆黑，将左子、左左子之一变红。
fn move_red_left<K, V>(mut h: Link<K, V>) -> Link<K, V> {
    flip_colors_for_del(h);

    if is_red(left_of(right_of(h))) {
        h.map(|mut n| unsafe {
            n.as_mut().right = rotate_right(right(n));
        });
        h = rotate_left(h);
    }

    h
}

/// 设置为父黑子红。
fn flip_colors_for_del<K, V>(h: Link<K, V>) {
    set_color(h, Color::Black);
    set_color(left_of(h), Color::Red);
    set_color(right_of(h), Color::Red);
}

/// 插入新节点。
fn put_with<K, V>(mut h: Link<K, V>, key: K, value: V) -> Link<K, V>
where
    K: PartialOrd,
{
    if h.is_none() {
        let n = Node::new(key, value, 1, Color::Red);
        return Some(Box::leak(Box::new(n)).into());
    }

    if let Some(mut n) = h {
        let n = unsafe { n.as_mut() };
        if key == n.key {
            n.value = value;
        } else if key > n.key {
            n.right = put_with(n.right, key, value);
        } else {
            n.left = put_with(n.left, key, value);
        }

        if is_red(n.right) && !is_red(n.left) {
            h = rotate_left(h);
        }
    }

    if is_red(left_of(h)) && is_red(left_of(left_of(h))) {
        h = rotate_right(h);
    }

    if is_red(left_of(h)) && is_red(right_of(h)) {
        flip_colors(h);
    }

    h.map(|mut n| unsafe {
        n.as_mut().update_size();
    });

    h
}

/// 左旋，将 h 变为左子树。
fn rotate_left<K, V>(h: Link<K, V>) -> Link<K, V> {
    let x = right_of(h);
    x.map(|mut n| unsafe {
        // 有子节点，则它本身非空。
        let ph = h.unwrap().as_mut();
        let px = n.as_mut();

        ph.right = px.left;
        px.left = h;
        px.color = ph.color;
        ph.color = Color::Red;
        px.len = ph.len;
        ph.update_size();
    });
    x
}

/// 右旋，将 h 变为右子树。
fn rotate_right<K, V>(h: Link<K, V>) -> Link<K, V> {
    let x = left_of(h);
    x.map(|mut n| unsafe {
        // 有子节点，则其本身非空。
        let ph = h.unwrap().as_mut();
        let px = n.as_mut();

        ph.left = px.right;
        px.right = h;
        px.color = ph.color;
        ph.color = Color::Red;
        px.len = ph.len;
        ph.update_size();
    });
    x
}

/// 反转 h 连接的颜色。
fn flip_colors<K, V>(h: Link<K, V>) {
    set_color(h, Color::Red);
    set_color(left_of(h), Color::Black);
    set_color(right_of(h), Color::Black);
}

/// 是否为红连接。
fn is_red<K, V>(h: Link<K, V>) -> bool {
    h.map_or(false, |n| unsafe { n.as_ref().color == Color::Red })
}

/// 设置连接颜色。
fn set_color<K, V>(h: Link<K, V>, color: Color) {
    h.map(|mut n| unsafe { n.as_mut().color = color });
}

/// 获取节点长度。
fn size_of<K, V>(h: Link<K, V>) -> usize {
    h.map_or(0, |n| unsafe { n.as_ref().len })
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> RedBlackBST<u8, char> {
        let mut t = RedBlackBST::new();
        ['s', 'e', 'a', 'r', 'c', 'h', 'x', 'm', 'p', 'l']
            .iter()
            .for_each(|&ch| {
                t.put(ch as u8, ch);
            });
        println!("tree:{}", &t);
        t
    }

    #[test]
    fn put_test() {
        let t = create_tree();
        assert_eq!(10, size_of(t.root));

        println!("遍历：");
        for i in t.iter() {
            println!("{}, {}, {:?}", i.key, i.value, i.color);
        }
    }

    #[test]
    fn delete_min_test() {
        let mut t = create_tree();

        let n = get_from(t.root, b'a');
        assert!(n.is_some());

        t.delete_min();
        assert_eq!(9, size_of(t.root));

        let n = get_from(t.root, b'a');
        assert!(n.is_none());

        println!("删除最小节点后：{t}");
    }

    #[test]
    fn delete_max_test() {
        let mut t = create_tree();

        let n = get_from(t.root, b'x');
        assert!(n.is_some());

        t.delete_max();
        assert_eq!(9, size_of(t.root));

        let n = get_from(t.root, b'x');
        assert!(n.is_none());
    }

    #[test]
    fn delete_test() {
        let mut t = create_tree();

        let n = get_from(t.root, b'x');
        assert!(n.is_some());

        t.delete(b'x');
        assert_eq!(9, size_of(t.root));

        let n = get_from(t.root, b'x');
        assert!(n.is_none());
    }
}
