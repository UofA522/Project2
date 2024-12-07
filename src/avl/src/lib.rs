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