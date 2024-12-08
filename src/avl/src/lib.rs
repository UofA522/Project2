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
}

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
        }))
    }
}

