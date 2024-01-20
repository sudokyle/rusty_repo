use std::cmp::Ordering;
use std::fmt::Display;

pub struct Node<T: Ord+Display> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T
}

impl<T: Ord+Display> Node<T> {
    pub fn new(value: T) -> Self {
        return Node{value, left: None, right: None}
    }

    pub fn insert(&mut self, value: T) {
        let mut node = self;
        loop {
            if value == node.value {
                return;
            }

            let child = match value.partial_cmp(&node.value).expect("Failed to make comparison") {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return,
            };

            match child {
                Some(ref mut c) => node = c, // ref keyword allows pattern to reference instead of consume the value.
                None => {
                    *child = Some(Box::new(Node::new(value)));
                    return;
                }
            }
        }
    }

    pub fn take_minimum(&mut self) -> Option<Box<Node<T>>>{
        return match self.left {
            Some(ref mut left) => {
                if let Some(node) = left.take_minimum() {
                    return Some(node);
                } else {
                    let mut l = self.left.take();
                    if let Some(ref mut l) = l {
                        self.left = std::mem::replace(&mut l.right, None);
                    }
                    return l;
                }
            }
            None => None,
        };
    }

    pub fn delete(mut node: Box<Node<T>>, value: &T) -> Option<Box<Node<T>>>{
        if value.cmp(&node.value) == Ordering::Less {
            if let Some(left) = node.left.take() {
                node.left = Node::delete(left, value);
            }
            return Some(node);
        }

        if value.cmp(&node.value) == Ordering::Greater {
            if let Some(right) = node.right.take() {
                node.right = Node::delete(right, value);
            }
            return Some(node);
        }

        assert!(&node.value == value, "Ord operation failed during Binary Tree deletion");

        match (node.left.take(), node.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(mut right)) => {
                if let Some(mut minimum) = right.take_minimum() {
                    minimum.left = Some(left);
                    minimum.right = Some(right);
                    return Some(minimum); // i.e. left most
                } else {
                    right.left = Some(left);
                    return Some(right);
                }
            }
        }
    }

    pub fn in_order_walk(node: &Option<Box<Node<T>>>, depth: isize) {
        if let Some(node) = node {
            Node::in_order_walk(&node.left, depth+1);

            let mut tabs = String::new();
            for _ in 0..depth {
                tabs.push_str("\t");
            }
            println!("{tabs}{}", node.value);

            Node::in_order_walk(&node.right, depth+1);
        }
    }
}

pub struct BinaryTree<T: Ord+Display> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord+Display> BinaryTree<T> {
    pub fn new() -> Self {
        return BinaryTree{root: None}
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    pub fn delete(&mut self, value: &T) {
        if let Some(root) = self.root.take() {
            self.root = Node::delete(root, value);
        }
    }

    pub fn in_order_walk(&self) {
        Node::in_order_walk(&self.root, 0);
    }
}