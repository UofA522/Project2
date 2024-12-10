use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::rc::{Rc, Weak};
use common::{DotNodeColor, Dotfile};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type WeakTree<T> = Weak<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;


#[derive(Debug)]
pub struct TreeNode<T> {
    color: NodeColor,
    key: T,
    parent: Option<WeakTree<T>>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

impl<T: Ord + Clone + std::fmt::Debug> TreeNode<T> {
    pub fn new(key: T) -> Tree<T> {
        Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }

    fn get_dot_color(color:NodeColor) -> DotNodeColor{
        match color{
            NodeColor::Red => {
                DotNodeColor::Red
            }
            NodeColor::Black => {
                DotNodeColor::Black
            }
        }
    }

    fn draw_node(node: &RedBlackTree<T>, file: &mut Dotfile, mut parent_node_idx: Option<usize>) {
        if let Some(root) = node {
            let root_node = match parent_node_idx {
                None => { file.add_node(format!("{:?}", root.clone().borrow().key.clone()).as_str(), TreeNode::<T>::get_dot_color(root.clone().borrow().color.clone())) }
                Some(parent_val) => {
                    parent_val
                }
            };

            if let Some(left) = root.clone().borrow().left.clone() {
                let left_node = file.add_node(format!("{:?}", left.borrow().key.clone()).as_str(), TreeNode::<T>::get_dot_color(left.borrow().color.clone()));
                file.add_edge(root_node, left_node);
                Self::draw_node(&root.clone().borrow().left.clone(), file, Some(left_node));
            } else {
                let left_node = file.add_node("None", DotNodeColor::Black);
                file.add_edge(root_node, left_node);
            }
            if let Some(right) = root.clone().borrow().right.clone() {
                let right_node = file.add_node(format!("{:?}", right.borrow().key.clone()).as_str(), TreeNode::<T>::get_dot_color(right.borrow().color.clone()));
                file.add_edge(root_node, right_node);
                Self::draw_node(&root.clone().borrow().right.clone(), file, Some(right_node));
            } else {
                let right_node = file.add_node("None", DotNodeColor::Black);
                file.add_edge(root_node, right_node);
            }
        }
    }
}
#[derive(Debug)]
pub struct RedBlackTreeStructure<T> {
    pub root: RedBlackTree<T>,
}

impl<T: Ord + std::fmt::Debug + std::fmt::Display + std::clone::Clone> RedBlackTreeStructure<T> {
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
                    if parent.borrow().right.is_some() && Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                        drop(parent.clone());
                        drop(node);
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
    pub fn number_of_leaves(root: &RedBlackTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }
        if root.clone().unwrap().borrow().left.is_none() && root.clone().unwrap().borrow().right.is_none() {
            return 1;
        }
        return RedBlackTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().left.clone()) + RedBlackTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().right.clone());
    }

    pub fn height_of_tree(root: &RedBlackTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }

        let left_height = Self::height_of_tree(&root.clone().unwrap().borrow().left.clone());
        let right_height = Self::height_of_tree(&root.clone().unwrap().borrow().right.clone());
        std::cmp::max(left_height, right_height) + 1
    }

    pub fn in_order_traversal(root: &RedBlackTree<T>) {
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

    pub fn draw_tree(&self, file: &mut Dotfile) {
        if self.root.is_none() {
            println!("There is nothing to draw")
        }
        TreeNode::draw_node(&self.root.clone(), file, None)
    }

    fn find_node(&self, key: T) -> RedBlackTree<T> {
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

    fn find_min(&self, mut node: Tree<T>) -> Tree<T> {
        let mut current_node = node.clone();
        while let Some(left) = current_node.clone().borrow().left.clone() {
            current_node = left.clone();
        }
        node
    }

    fn fix_delete(&mut self, mut node: Tree<T>) {
        while let Some(parent) = node.borrow().parent.clone().and_then(|p| p.upgrade()) {
            if parent.borrow().color == NodeColor::Black {
                break;
            }

            let is_left = Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap());
            let sibling = if is_left {
                parent.borrow().right.clone()
            } else {
                parent.borrow().left.clone()
            };

            if let Some(sibling_node) = sibling {
                if sibling_node.borrow().color == NodeColor::Red {
                    sibling_node.borrow_mut().color = NodeColor::Black;
                    parent.borrow_mut().color = NodeColor::Red;
                    if is_left {
                        self.rotate_left(parent.clone());
                    } else {
                        self.rotate_right(parent.clone());
                    }
                } else {
                    self.handle_black_sibling_case(node.clone(), sibling_node.clone(), parent.clone());
                }
            }
        }

        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
    }

    fn handle_black_sibling_case(&mut self, node: Tree<T>, sibling: Tree<T>, parent: Tree<T>) {
        let left_black = sibling.borrow().left.as_ref().map_or(true, |left| left.borrow().color == NodeColor::Black);
        let right_black = sibling.borrow().right.as_ref().map_or(true, |right| right.borrow().color == NodeColor::Black);

        if left_black && right_black {
            sibling.borrow_mut().color = NodeColor::Red;
            if parent.borrow().color == NodeColor::Black {
                self.fix_delete_double_black(node);
            } else {
                parent.borrow_mut().color = NodeColor::Black;
            }
        } else {
            self.fix_red_sibling_case(node, sibling, parent);
        }
    }

    fn fix_red_sibling_case(&mut self, node: Tree<T>, sibling: Tree<T>, parent: Tree<T>) {
        let is_left = Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap());
        if is_left && sibling.borrow().right.as_ref().map_or(false, |n| n.borrow().color == NodeColor::Red) {
            self.rotate_left(sibling.clone());
        } else if !is_left && sibling.borrow().left.as_ref().map_or(false, |n| n.borrow().color == NodeColor::Red) {
            self.rotate_right(sibling.clone());
        }

        sibling.borrow_mut().color = parent.borrow().color.clone();
        parent.borrow_mut().color = NodeColor::Black;

        if is_left {
            sibling.borrow_mut().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
            self.rotate_left(parent.clone());
        } else {
            sibling.borrow_mut().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
            self.rotate_right(parent.clone());
        }
    }



    fn delete_node(&mut self, node: Tree<T>) {
        let mut to_fix = node.clone();

        if node.borrow().left.is_some() && node.borrow().right.is_some() {
            let successor = self.find_min(node.borrow().right.clone().unwrap());
            node.borrow_mut().key = successor.borrow().key.clone();
            to_fix = successor.clone();
        }
        let replacement = if to_fix.borrow().left.is_some() {
            to_fix.borrow().left.clone()
        } else {
            to_fix.borrow().right.clone()
        };
        if let Some(replacement_node) = replacement.clone() {
            replacement_node.borrow_mut().parent = to_fix.borrow().parent.clone();
            if let Some(parent) = to_fix.borrow().parent.clone().and_then(|p| p.upgrade()) {
                if Rc::ptr_eq(&to_fix, &parent.borrow().left.as_ref().unwrap()) {
                    parent.borrow_mut().left = replacement.clone();
                } else {
                    parent.borrow_mut().right = replacement.clone();
                }
            } else {
                self.root = replacement.clone();
            }
        } else if let Some(parent) = to_fix.borrow().parent.clone().and_then(|p| p.upgrade()) {
            if Rc::ptr_eq(&to_fix, &parent.borrow().left.as_ref().unwrap()) {
                parent.borrow_mut().left = None;
            } else {
                parent.borrow_mut().right = None;
            }
        } else {
            self.root = None;
        }

        if to_fix.borrow().color == NodeColor::Black {
            if let Some(replacement_node) = replacement {
                self.fix_delete(replacement_node);
            } else {
                self.fix_delete_double_black(to_fix);
            }
        }
    }


    fn fix_double_red(&mut self, mut node: Tree<T>) {
        while let Some(parent) = node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            if parent.borrow().color == NodeColor::Black {
                break;
            }

            if let Some(grandparent) = parent.borrow().parent.clone().and_then(|p| p.upgrade()) {
                let is_left = Rc::ptr_eq(&parent, &grandparent.borrow().left.as_ref().unwrap());
                let uncle = if is_left {
                    grandparent.borrow().right.clone()
                } else {
                    grandparent.borrow().left.clone()
                };

                if let Some(uncle_node) = uncle {
                    if uncle_node.borrow().color == NodeColor::Red {
                        parent.borrow_mut().color = NodeColor::Black;
                        uncle_node.borrow_mut().color = NodeColor::Black;
                        grandparent.borrow_mut().color = NodeColor::Red;
                        drop(node);
                        node = grandparent;
                    } else {
                        if is_left {
                            if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                                self.rotate_left(parent.clone());
                                node = parent.clone();
                            }
                            parent.borrow_mut().color = NodeColor::Black;
                            grandparent.borrow_mut().color = NodeColor::Red;
                            self.rotate_right(grandparent.clone());
                        } else {
                            if Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                                self.rotate_right(parent.clone());
                                node = parent.clone();
                            }
                            parent.borrow_mut().color = NodeColor::Black;
                            grandparent.borrow_mut().color = NodeColor::Red;
                            self.rotate_left(grandparent.clone());
                        }
                        break;
                    }
                } else {
                    if is_left {
                        if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                            self.rotate_left(parent.clone());
                            node = parent.clone();
                        }
                        parent.borrow_mut().color = NodeColor::Black;
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_right(grandparent.clone());
                    } else {
                        if Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                            self.rotate_right(parent.clone());
                            node = parent.clone();
                        }
                        parent.borrow_mut().color = NodeColor::Black;
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_left(grandparent.clone());
                    }
                    break;
                }
            } else {
                break;
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
    }






    pub fn delete(&mut self, key: T) {
        if let Some(node) = self.find_node(key) {
            self.delete_node(node)
        }
    }

    fn fix_delete_double_black(&mut self, mut node: Tree<T>) {
        while let Some(parent) = node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
            let is_left:bool = if parent.borrow().left.is_some() {
                Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap())
            }
            else { 
                false
            };
            let sibling = if is_left {
                parent.borrow().right.clone()
            } else {
                parent.borrow().left.clone()
            };

            if let Some(sibling_node) = sibling {
                if sibling_node.borrow().color == NodeColor::Red {
                    sibling_node.borrow_mut().color = NodeColor::Black;
                    parent.borrow_mut().color = NodeColor::Red;
                    if is_left {
                        self.rotate_left(parent.clone());
                    } else {
                        self.rotate_right(parent.clone());
                    }
                } else {
                    let left_black = sibling_node
                        .borrow()
                        .left
                        .as_ref()
                        .map_or(true, |left| left.borrow().color == NodeColor::Black);
                    let right_black = sibling_node
                        .borrow()
                        .right
                        .as_ref()
                        .map_or(true, |right| right.borrow().color == NodeColor::Black);

                    if left_black && right_black {
                        sibling_node.borrow_mut().color = NodeColor::Red;
                        if parent.borrow().color == NodeColor::Black {
                            drop(node);
                            node = parent.clone();
                        } else {
                            parent.borrow_mut().color = NodeColor::Black;
                            break;
                        }
                    } else {
                        if is_left && right_black {
                            sibling_node.borrow_mut().color = NodeColor::Red;
                            sibling_node.borrow_mut().left.as_ref().unwrap().borrow_mut().color =
                                NodeColor::Black;
                            self.rotate_right(sibling_node.clone());
                        } else if !is_left && left_black {
                            sibling_node.borrow_mut().color = NodeColor::Red;
                            sibling_node.borrow_mut().right.as_ref().unwrap().borrow_mut().color =
                                NodeColor::Black;
                            self.rotate_left(sibling_node.clone());
                        }

                        sibling_node.borrow_mut().color = parent.borrow().color.clone();
                        parent.borrow_mut().color = NodeColor::Black;

                        if is_left {
                            sibling_node
                                .borrow_mut()
                                .right
                                .as_ref()
                                .unwrap()
                                .borrow_mut()
                                .color = NodeColor::Black;
                            self.rotate_left(parent.clone());
                        } else {
                            sibling_node
                                .borrow_mut()
                                .left
                                .as_ref()
                                .unwrap()
                                .borrow_mut()
                                .color = NodeColor::Black;
                            self.rotate_right(parent.clone());
                        }

                        break;
                    }
                }
            } else {
                break;
            }
        }

        // Finally, make the node black
        node.borrow_mut().color = NodeColor::Black;
    }

    fn find_mininmum_node(&self, node: Tree<T>) -> Tree<T> {
        let mut current = node.clone();
        while let Some(left) = current.clone().borrow().left.clone() {
            current = left.clone()
        }
        current
    }

    pub fn find_by_key(&self, key:T) -> RedBlackTree<T> {
        if self.tree_is_empty() {
            None
        }
        else {
            self.find_node(key)
        }
    }
}