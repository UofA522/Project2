use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type WeakTree<T> = Weak<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;


#[derive(Debug)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: Option<WeakTree<T>>,
    pub left: RedBlackTree<T>,
    pub right: RedBlackTree<T>,
}

impl<T: Ord + Clone> TreeNode<T> {
    pub fn new(key: T) -> Tree<T> {
        Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }
}
#[derive(Debug)]
struct RedBlackTreeStructure<T> {
    root: RedBlackTree<T>,
}

impl<T: Ord + std::fmt::Debug> RedBlackTreeStructure<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T)
    where
        T: Clone + Ord,
    {
        let new_node = TreeNode::new(key.clone());

        if self.root.is_none() {
            new_node.borrow_mut().color = NodeColor::Black;
            self.root = Some(new_node);
        } else {
            let mut current = self.root.clone();
            let mut parent = None;

            while let Some(node) = current {
                let cmp = key.cmp(&node.borrow().key);
                parent = Some(node.clone());
                current = match cmp {
                    Ordering::Less => node.borrow().left.clone(),
                    Ordering::Greater | Ordering::Equal => node.borrow().right.clone(),
                };
            }

            if let Some(parent_node) = parent {
                new_node.borrow_mut().parent = Some(Rc::downgrade(&parent_node));
                if key < parent_node.borrow().key {
                    parent_node.borrow_mut().left = Some(new_node.clone());
                } else {
                    parent_node.borrow_mut().right = Some(new_node.clone());
                }
            }

            self.fix_insert(new_node);
        }
    }

    fn fix_insert(&mut self, mut node: Tree<T>) {
        while let Some(parent) = node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            if parent.borrow().color != NodeColor::Red {
                break;
            }

            let grandparent = parent.borrow().parent.clone().and_then(|gp| gp.upgrade());
            if let Some(grandparent_node) = grandparent {
                let is_left = match grandparent_node.borrow().left.as_ref() {
                    Some(left) if Rc::ptr_eq(&parent, left) => true,
                    _ => false,
                };

                let uncle = if is_left {
                    grandparent_node.borrow().right.clone()
                } else {
                    grandparent_node.borrow().left.clone()
                };

                if let Some(uncle_node) = uncle {
                    if uncle_node.borrow().color == NodeColor::Red {
                        // Recolor
                        parent.borrow_mut().color = NodeColor::Black;
                        uncle_node.borrow_mut().color = NodeColor::Black;
                        grandparent_node.borrow_mut().color = NodeColor::Red;
                        drop(node);
                        node = grandparent_node.clone();
                        continue;
                    }
                }

                if is_left {
                    if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                        self.rotate_left(parent.clone());
                        node = parent.clone();
                    }
                    self.rotate_right(grandparent_node.clone());
                } else {
                    if parent.borrow().left.as_ref().is_some() && Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                        self.rotate_right(parent.clone());
                        node = parent.clone();
                    }
                    self.rotate_left(grandparent_node.clone());
                }

                parent.borrow_mut().color = NodeColor::Black;
                grandparent_node.borrow_mut().color = NodeColor::Red;
            }
        }

        if let Some(root) = &self.root {
            root.borrow_mut().color = NodeColor::Black;
        }
    }

    fn rotate_left(&mut self, node: Tree<T>) {
        let right = node.borrow_mut().right.take().unwrap();
        node.borrow_mut().right = right.borrow_mut().left.take();

        if let Some(left_child) = &node.borrow().right {
            left_child.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        let parent = node.borrow_mut().parent.clone();
        right.borrow_mut().parent = parent.clone();

        if let Some(parent_node) = parent.and_then(|p| p.upgrade()) {
            if Rc::ptr_eq(&node, &parent_node.borrow().left.as_ref().unwrap()) {
                parent_node.borrow_mut().left = Some(right.clone());
            } else {
                parent_node.borrow_mut().right = Some(right.clone());
            }
        } else {
            self.root = Some(right.clone());
        }

        right.borrow_mut().left = Some(node.clone());
        node.borrow_mut().parent = Some(Rc::downgrade(&right));
    }

    fn rotate_right(&mut self, node: Tree<T>) {
        let left = node.borrow_mut().left.take().unwrap();
        node.borrow_mut().left = left.borrow_mut().right.take();

        if let Some(right_child) = &node.borrow().left {
            right_child.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        let parent = node.borrow_mut().parent.clone();
        left.borrow_mut().parent = parent.clone();

        if let Some(parent_node) = parent.and_then(|p| p.upgrade()) {
            if Rc::ptr_eq(&node, &parent_node.borrow().left.as_ref().unwrap()) {
                parent_node.borrow_mut().left = Some(left.clone());
            } else {
                parent_node.borrow_mut().right = Some(left.clone());
            }
        } else {
            self.root = Some(left.clone());
        }

        left.borrow_mut().right = Some(node.clone());
        node.borrow_mut().parent = Some(Rc::downgrade(&left));
    }
}

fn main() {
    let mut rb_tree = RedBlackTreeStructure::new();
    rb_tree.insert(10);
    rb_tree.insert(20);
    rb_tree.insert(30);
    rb_tree.insert(15);
    rb_tree.insert(25);
    rb_tree.insert(5);
    rb_tree.insert(6);
    rb_tree.insert(1);


    println!("{:#?}", rb_tree.root);
}
