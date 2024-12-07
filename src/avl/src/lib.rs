use std::cell::RefCell;
use std::rc::{Rc, Weak};
//#[derive(Clone, Debug, PartialEq)]

type AVLTreeStrong<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLTree<T>= Option<AVLTreeStrong<T>>;

type AVLWeakTree<T> = Weak<RefCell<AVLTreeNode<T>>>;

#[derive(Debug)]
struct AVLTreeNode<T> {
    pub key: T,
    pub parent: Option<AVLWeakTree<T>>,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

pub struct AVLTreeStructure<T> {
    pub root: AVLTree<T>
}


impl<T: Ord> AVLTreeNode<T> {
    fn new(key: T) -> AVLTreeStrong<T> {
        Rc::new(RefCell::new(AVLTreeNode {
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

impl<T: Ord> AVLTreeStructure<T> {

    pub fn new() -> Self {
        AVLTreeStructure {
            root:None
        }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = AVLTreeNode::new(key);

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }
        let inserted_node = self.insert_node(self.root.clone().unwrap(), new_node);
        if let Some(parent) = inserted_node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            self.rebalance(parent);
        }
    }

    fn insert_node(
        &mut self,
        current: AVLTreeStrong<T>,
        new_node: AVLTreeStrong<T>,
    ) -> AVLTreeStrong<T> {
        if new_node.borrow().key < current.borrow().key {
            // Go to the left subtree
            if let Some(left) = current.borrow().left.clone() {
                self.insert_node(left, new_node)
            } else {
                current.borrow_mut().left = Some(new_node.clone());
                new_node.borrow_mut().parent = Some(Rc::downgrade(&current));
                new_node
            }
        } else {
            if let Some(right) = current.borrow().right.clone() {
                self.insert_node(right, new_node)
            } else {
                current.borrow_mut().right = Some(new_node.clone());
                new_node.borrow_mut().parent = Some(Rc::downgrade(&current));
                new_node
            }
        }
    }
    fn rebalance(&mut self, mut node: AVLTreeStrong<T>) {
        while let Some(parent) = node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            let balance_factor = self.get_balance(parent.clone());

            if balance_factor > 1 {
                if self.get_balance(parent.borrow().left.clone().unwrap()) < 0 {
                    self.rotate_left(parent.borrow().left.clone().unwrap());
                }
                self.rotate_right(parent.clone());
            } else if balance_factor < -1 {
                if self.get_balance(parent.borrow().right.clone().unwrap()) > 0 {
                    self.rotate_right(parent.borrow().right.clone().unwrap());
                }
                self.rotate_left(parent.clone());
            }
            drop(node);
            node = parent;
        }
    }

    fn get_balance(&self, node: AVLTreeStrong<T>) -> i32 {
        self.get_height(node.borrow().left.clone())
            - self.get_height(node.borrow().right.clone())
    }
    fn get_height(&self, subtree: AVLTree<T>) -> i32 {
        if let Some(node) = subtree {
            1 + std::cmp::max(
                self.get_height(node.borrow().left.clone()),
                self.get_height(node.borrow().right.clone()),
            )
        } else {
            0
        }
    }

    fn rotate_left(&mut self, node: AVLTreeStrong<T>) {
        let right = node.borrow_mut().right.take().unwrap();

        // Adjust subtree references
        node.borrow_mut().right = right.borrow_mut().left.take();
        if let Some(left) = &node.borrow().right {
            left.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        let parent = node.borrow().parent.clone();
        right.borrow_mut().parent = parent.clone();

        // Adjust parent's reference to the new root
        if let Some(parent) = parent.and_then(|p| p.upgrade()) {
            if Rc::ptr_eq(&node, parent.borrow().left.as_ref().unwrap()) {
                parent.borrow_mut().left = Some(right.clone());
            } else {
                parent.borrow_mut().right = Some(right.clone());
            }
        } else {
            self.root = Some(right.clone());
        }
        right.borrow_mut().left = Some(node.clone());
        node.borrow_mut().parent = Some(Rc::downgrade(&right));
    }
    fn rotate_right(&mut self, node: AVLTreeStrong<T>) {
        let left = node.borrow_mut().left.take().unwrap();
        node.borrow_mut().left = left.borrow_mut().right.take();
        if let Some(right) = &node.borrow().left {
            right.borrow_mut().parent = Some(Rc::downgrade(&node));
        }
        let parent = node.borrow().parent.clone();
        left.borrow_mut().parent = parent.clone();
        if let Some(parent) = parent.and_then(|p| p.upgrade()) {
            if Rc::ptr_eq(&node, parent.borrow().left.as_ref().unwrap()) {
                parent.borrow_mut().left = Some(left.clone());
            } else {
                parent.borrow_mut().right = Some(left.clone());
            }
        } else {
            self.root = Some(left.clone());
        }
        left.borrow_mut().right = Some(node.clone());
        node.borrow_mut().parent = Some(Rc::downgrade(&left));
    }
}


