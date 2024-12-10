use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};
use common::{DotNodeColor, Dotfile};

type AVLTreeStrong<T> = Rc<RefCell<AVLTreeNode<T>>>;
type AVLTree<T> = Option<AVLTreeStrong<T>>;
type AVLWeakTree<T> = Weak<RefCell<AVLTreeNode<T>>>;

#[derive(Debug)]
pub struct AVLTreeNode<T> {
    pub key: T,
    pub parent: Option<AVLWeakTree<T>>,
    pub left: AVLTree<T>,
    pub right: AVLTree<T>,
    pub height: i32
}

#[derive(Debug)]
pub struct AVLTreeStructure<T> {
    pub root: AVLTree<T>,
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

impl<T: Ord + Clone + std::fmt::Debug> AVLTreeNode<T> {
    fn height(node: &AVLTree<T>) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node_ref = node.as_ref().unwrap(); // Unwrap the node safely
        let left_height = Self::height(&node_ref.borrow().left);
        let right_height = Self::height(&node_ref.borrow().right);

        std::cmp::max(left_height, right_height) + 1
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

    fn balance(node: &AVLTreeStrong<T>) -> AVLTreeStrong<T> {
        node.borrow_mut().update_height();
        let balance = node.borrow().balance_factor();
        if balance > 1 {
            if node.borrow().left.clone().unwrap().borrow().balance_factor() < 0 {
                let mut node_borrow = node.borrow_mut();
                node_borrow.left = Some(Self::rotate_left(
                    node_borrow.left.clone().unwrap().clone(),
                ));
            }
            return Self::rotate_right(node.clone());
        } else if balance < -1 {
            if node.borrow().right.clone().unwrap().borrow().balance_factor() > 0 {
                let mut node_borrow = node.borrow_mut();
                node_borrow.right = Some(Self::rotate_right(
                    node_borrow.right.clone().unwrap().clone(),
                ));
            }
            return Self::rotate_left(node.clone());
        }
        node.clone()
    }

    fn draw_node(node: &AVLTree<T>, file: &mut Dotfile, mut parent_node_idx: Option<usize>) {
        if let Some(root) = node {
            let root_node = match parent_node_idx {
                None => { file.add_node(format!("{:?}", root.clone().borrow().key.clone()).as_str(), DotNodeColor::Green) }
                Some(parent_val) => {
                    parent_val
                }
            };

            if let Some(left) = root.clone().borrow().left.clone() {
                let left_node = file.add_node(format!("{:?}", left.borrow().key.clone()).as_str(), DotNodeColor::Green);
                file.add_edge(root_node, left_node);
                Self::draw_node(&root.clone().borrow().left.clone(), file, Some(left_node));
            } else {
                let left_node = file.add_node("None", DotNodeColor::Green);
                file.add_edge(root_node, left_node);
            }
            if let Some(right) = root.clone().borrow().right.clone() {
                let right_node = file.add_node(format!("{:?}", right.borrow().key.clone()).as_str(), DotNodeColor::Green);
                file.add_edge(root_node, right_node);
                Self::draw_node(&root.clone().borrow().right.clone(), file, Some(right_node));
            } else {
                let right_node = file.add_node("None", DotNodeColor::Green);
                file.add_edge(root_node, right_node);
            }
        }
    }
}

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> AVLTreeStructure<T> {

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
        AVLTreeNode::balance(&node)
    }

    pub fn insert(&mut self, key: T) {
        if let Some(root) = self.root.clone() {
            self.root = Some(Self::insert_node(root, key));
        } else {
            self.root = Some(AVLTreeNode::new(key));
        }
    }

    pub fn delete(&mut self, key: T) {
        if let Some(root) = self.root.clone() {
            self.root = Self::delete_node(root, key);
        }
    }

    fn delete_node(node: AVLTreeStrong<T>, key: T) -> AVLTree<T> {
        let mut node_borrow = node.borrow_mut();

        if key < node_borrow.key {
            if node_borrow.left.is_some() {
                let left = node_borrow.left.take().unwrap();
                node_borrow.left = Self::delete_node(left, key);
            }
        } else if key > node_borrow.key {
            if node_borrow.right.is_some() {
                let right = node_borrow.right.take().unwrap();
                node_borrow.right = Self::delete_node(right, key);
            }
        } else {
            if node_borrow.left.is_none() && node_borrow.right.is_none() {
                return None;
            } else if node_borrow.left.is_none() {
                return node_borrow.right.take();
            } else if node_borrow.right.is_none() {
                return node_borrow.left.take();
            } else {
                if let Some(successor) = Self::min_node(node_borrow.right.clone()) {
                    node_borrow.key = successor.borrow().key.clone();
                    node_borrow.right = Self::delete_node(successor.clone(), successor.borrow().key.clone());
                }
            }
        }

        drop(node_borrow);
        Some(AVLTreeNode::balance(&node))
    }




    fn min_node(node: AVLTree<T>) -> Option<AVLTreeStrong<T>> {
        let mut current = node;
        while let Some(ref n) = current.clone().unwrap().borrow().left {
            current = Some(n.clone());
        }
        current
    }

    pub fn draw_tree(&self, file: &mut Dotfile) {
        if self.root.is_none() {
            println!("There is nothing to draw")
        }
        AVLTreeNode::draw_node(&self.root.clone(), file, None)
    }

    pub fn number_of_leaves(root: &AVLTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }
        if root.clone().unwrap().borrow().left.is_none() && root.clone().unwrap().borrow().right.is_none() {
            return 1;
        }
        return AVLTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().left.clone()) + AVLTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().right.clone());
    }
    pub fn height_of_tree(root: &AVLTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }

        let left_height = Self::height_of_tree(&root.clone().unwrap().borrow().left.clone());
        let right_height = Self::height_of_tree(&root.clone().unwrap().borrow().right.clone());
        std::cmp::max(left_height, right_height) + 1
    }

    pub fn in_order_traversal(root: &AVLTree<T>) {
        if root.is_some() {
            Self::in_order_traversal(&root.clone().unwrap().borrow().left.clone());
            println!("{}", root.clone().unwrap().borrow().key);
            Self::in_order_traversal(&root.clone().unwrap().borrow().right.clone());
        }
    }
    pub fn tree_is_empty(&self) -> bool {
        if self.root.is_none() {
            return true;
        }
        false
    }

    fn find_node(&self,key: T) -> AVLTree<T>{
        let mut current = self.root.clone();
        while let Some(node) = current.clone() {
            match key.cmp(&node.clone().borrow().key) {
                Ordering::Less => {
                    current = node.borrow().left.clone();
                }
                Ordering::Equal => {
                    return Some(node)
                }
                Ordering::Greater => {
                    current = node.borrow().right.clone();
                }
            }
        }
        None
    }
    pub fn find_by_key(&self, key:T) -> AVLTree<T> {
        if self.tree_is_empty() {
            None
        }
        else {
            self.find_node(key)
        }
    }
}


