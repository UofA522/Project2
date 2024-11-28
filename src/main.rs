use std::cell::{Ref, RefCell, RefMut};
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

    fn recolor(mut node: RefMut<TreeNode<T>>) {
        node.color = match node.color {
            NodeColor::Red => NodeColor::Black,
            NodeColor::Black => NodeColor::Red,
            NodeColor::None => NodeColor::None,
        };
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

    fn insert_fix(&self, new_node: Rc<RefCell<TreeNode<T>>>) {
        let mut current_node = new_node;
        while let Some(parent) = current_node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            if parent.borrow().color != NodeColor::Red {
                break;
            }
            println!("New Node's Parent is red ");
            let grandparent = parent.borrow().parent.clone().and_then(|gp| gp.upgrade());
            if let Some(grandparent_node) = grandparent {
                let parent_is_left_child_of_grandparent = match grandparent_node.borrow().left.as_ref() {
                    None => {
                        println!("No Left Child of Grandparent");
                        false
                    }
                    Some(left) => {
                        println!("Left child of Grandparent exists");
                        Rc::ptr_eq(&parent, left)
                    }
                };

                let uncle = if parent_is_left_child_of_grandparent {
                    println!("Uncle is Right child");
                    grandparent_node.borrow().right.clone()
                } else {
                    println!("Uncle is left child");
                    grandparent_node.borrow().left.clone()
                };

                if let Some(uncle_node) = uncle {
                    if uncle_node.borrow().color == NodeColor::Red {
                        println!("Performing Recolor");
                        TreeNode::recolor(parent.borrow_mut());
                        TreeNode::recolor(uncle_node.borrow_mut());
                        TreeNode::recolor(grandparent_node.borrow_mut());
                        drop(current_node);
                        current_node = grandparent_node.clone();
                        continue;
                    }
                }
            }
        }
    }


    fn insert_node(parent: &Rc<RefCell<TreeNode<T>>>, key: T) -> Rc<RefCell<TreeNode<T>>> {
        let child_to_insert = {
            let mut parent_borrow = parent.borrow_mut();

            match key.cmp(&parent_borrow.key) {
                Ordering::Less => {
                    if parent_borrow.left.is_none() {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent)));
                        parent_borrow.left = Some(new_node.clone());
                        return new_node;
                    }
                    parent_borrow.left.clone().unwrap()
                }
                Ordering::Greater | Ordering::Equal => {
                    if parent_borrow.right.is_none() {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent)));
                        parent_borrow.right = Some(new_node.clone());
                        return new_node;
                    }
                    parent_borrow.right.clone().unwrap()
                }
            }
        };
        RBTree::insert_node(&child_to_insert, key)
    }
}

impl<T: Ord + Clone + std::fmt::Debug> BasicFunction<T> for RBTree<T> {
    fn insert(&mut self, key: T) {
        let new_node = match &self.root {
            None => {
                let root = TreeNode::new(key.clone(), NodeColor::Black, None);
                self.root = Some(root.clone());
                root
            }
            Some(root_rc) => RBTree::insert_node(root_rc, key),
        };
        if let Some(root) = &self.root {
            self.insert_fix(new_node);
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
    root.insert(10);
    root.insert(8);
    root.insert(11);
    root.insert(7);
    println!("{:#?}", root);
}
