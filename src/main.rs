use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
    None,
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

impl<T: Ord> TreeNode<T> {
    fn new(key: T, color: NodeColor, parent: Option<WeakTree<T>>) -> Rc<RefCell<TreeNode<T>>> {
        Rc::new(RefCell::new(TreeNode {
            color,
            key,
            parent,
            left: None,
            right: None,
        }))
    }

    fn insert_node(&mut self, key: T, parent_rc: &Rc<RefCell<TreeNode<T>>>,parent_color:NodeColor) {
        match key.cmp(&self.key) {
            Ordering::Less => {
                match &self.left {
                    None => {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent_rc)));
                        self.left = Some(new_node);
                        if parent_color==NodeColor::Black{
                            return;
                        }
                    }
                    Some(left) => {
                        let mut left_borrow = left.borrow_mut();
                        let left_borrow_color = left_borrow.color.clone();
                        left_borrow.insert_node(key, left,left_borrow_color);
                    }
                }
            }
            Ordering::Equal | Ordering::Greater => {
                match &self.right {
                    None => {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent_rc)));
                        self.right = Some(new_node);
                    }
                    Some(right) => {
                        let mut right_borrow = right.borrow_mut();
                        let right_borrow_color = right_borrow.color.clone();
                        right_borrow.insert_node(key, right,right_borrow_color);
                    }
                }
            }
        }
    }
}

trait BasicFunction<T> {
    fn insert(&mut self, key: T);
    fn delete(&mut self, key: T);
    fn number_of_leaves(&self) -> u32;
    fn height_of_tree(&self) -> u32;
    fn inorder_traversal(&self);
    fn is_tree_empty(&self) -> bool;
    fn print_tree(&self);
}

#[derive(Debug)]
struct RBTree<T> {
    root: RedBlackTree<T>,
}

impl<T: Ord> RBTree<T> {
    fn new() -> Self {
        RBTree { root: None }
    }
}

impl<T: Ord + Clone + std::fmt::Debug> BasicFunction<T> for RBTree<T> {
    fn insert(&mut self, key: T) {
        match &self.root {
            None => {
                self.root = Some(TreeNode::new(key.clone(), NodeColor::Black, None));
            }
            Some(root_rc) => {
                let mut root_rc_borrow = root_rc.borrow_mut();
                let root_rc_color = root_rc_borrow.color.clone();
                root_rc_borrow.insert_node(key, root_rc,root_rc_color);
            }
        }

    }

    fn delete(&mut self, key: T) {
        todo!()
    }

    fn number_of_leaves(&self) -> u32 {
        todo!()
    }

    fn height_of_tree(&self) -> u32 {
        todo!()
    }

    fn inorder_traversal(&self) {
        todo!()
    }

    fn is_tree_empty(&self) -> bool {
        self.root.is_none()
    }

    fn print_tree(&self) {
        todo!()
    }
}

fn main() {
    let mut root = RBTree::new();
    root.insert(5);
    root.insert(1);
    root.insert(6);
    root.insert(7);
    root.insert(2);
    println!("{:#?}",root);

}
