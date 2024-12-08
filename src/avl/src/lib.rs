use std::cell::RefCell;
use std::rc::{Rc, Weak};

type AVLTreeStrong<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLTree<T> = Option<AVLTreeStrong<T>>;
type AVLWeakTree<T> = Weak<RefCell<AVLTreeNode<T>>>;

#[derive(Debug)]
struct AVLTreeNode<T> {
    pub key: T,
    pub parent: Option<AVLWeakTree<T>>,
    pub left: AVLTree<T>,
    pub right: AVLTree<T>,
    pub height: i32
}

#[derive(Debug)]
pub struct AVLTreeStructure<T> {
    root: AVLTree<T>,
}

impl<T: Ord> AVLTreeNode<T> {
    pub fn new(key: T) -> AVLTreeStrong<T> {
        Rc::new(RefCell::new(Self {
            key,
            parent: None,
            left: None,
            right: None,
            height:1
        }))
    }
}

impl<T: Ord> AVLTreeNode<T> {
    fn height(node: &AVLTree<T>) -> i32 {
        node.as_ref()
            .map(|n| AVLTreeNode::height(&n.borrow().left).max(AVLTreeNode::height(&n.borrow().right)) + 1)
            .unwrap_or(0)
    }

    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(
            AVLTreeNode::height(&self.left),
            AVLTreeNode::height(&self.right),
        );
    }

    fn balance_factor(&self) -> i32 {
        AVLTreeNode::height(&self.left) - AVLTreeNode::height(&self.right)
    }

    fn rotate_right(node: AVLTreeStrong<T>) -> AVLTreeStrong<T> {
        let left_child = node.borrow_mut().left.take().unwrap();
        node.borrow_mut().left = left_child.borrow_mut().right.take();
        if let Some(left_right) = &node.borrow().left {
            left_right.borrow_mut().parent = Some(Rc::downgrade(&node));
        }
        left_child.borrow_mut().right = Some(node.clone());
        left_child.borrow_mut().parent = node.borrow().parent.clone();
        node.borrow_mut().parent = Some(Rc::downgrade(&left_child));
        node.borrow_mut().update_height();
        left_child.borrow_mut().update_height();
        left_child
    }

    fn rotate_left(node: AVLTreeStrong<T>) -> AVLTreeStrong<T> {
        let right_child = node.borrow_mut().right.take().unwrap();
        node.borrow_mut().right = right_child.borrow_mut().left.take();
        if let Some(right_left) = &node.borrow().right {
            right_left.borrow_mut().parent = Some(Rc::downgrade(&node));
        }
        right_child.borrow_mut().left = Some(node.clone());
        right_child.borrow_mut().parent = node.borrow().parent.clone();
        node.borrow_mut().parent = Some(Rc::downgrade(&right_child));
        node.borrow_mut().update_height();
        right_child.borrow_mut().update_height();
        right_child
    }

    fn balance(node: AVLTreeStrong<T>) -> AVLTreeStrong<T> {
        node.borrow_mut().update_height();
        let balance = node.borrow().balance_factor();
        if balance > 1 {
            if node.borrow().left.as_ref().unwrap().borrow().balance_factor() < 0 {
                node.borrow_mut().left = Some(Self::rotate_left(
                    node.borrow().left.as_ref().unwrap().clone(),
                ));
            }
            return Self::rotate_right(node);
        } else if balance < -1 {
            if node.borrow().right.as_ref().unwrap().borrow().balance_factor() > 0 {
                node.borrow_mut().right = Some(Self::rotate_right(
                    node.borrow().right.as_ref().unwrap().clone(),
                ));
            }
            return Self::rotate_left(node);
        }
        node
    }
}

impl<T: Ord> AVLTreeStructure<T> {

    pub fn new() -> Self{
        AVLTreeStructure{
            root: None
        }
    }
    fn insert_node(node: AVLTreeStrong<T>, key: T) -> AVLTreeStrong<T> {
        if key < node.borrow().key {
            if node.borrow().left.is_some() {
                let updated_left = Self::insert_node(node.borrow().left.as_ref().unwrap().clone(), key);
                node.borrow_mut().left = Some(updated_left);
            } else {
                let new_node = AVLTreeNode::new(key);
                new_node.borrow_mut().parent = Some(Rc::downgrade(&node));
                node.borrow_mut().left = Some(new_node);
            }
        } else if key > node.borrow().key {
            if node.borrow().right.is_some() {
                let updated_right = Self::insert_node(node.borrow().right.as_ref().unwrap().clone(), key);
                node.borrow_mut().right = Some(updated_right);
            } else {
                let new_node = AVLTreeNode::new(key);
                new_node.borrow_mut().parent = Some(Rc::downgrade(&node));
                node.borrow_mut().right = Some(new_node);
            }
        }
        // Balance the tree and update height
        AVLTreeNode::balance(node)
    }

    pub fn insert(&mut self, key: T) {
        if let Some(root) = self.root.clone() {
            self.root = Some(Self::insert_node(root, key));
        } else {
            self.root = Some(AVLTreeNode::new(key));
        }
    }
}


