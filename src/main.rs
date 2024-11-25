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
    fn recolor(mut node: RefMut<TreeNode<T>>){
        if node.color == NodeColor::Red{
            node.color = NodeColor::Black;
        }
        else if node.color==NodeColor::Black {
            node.color= NodeColor::Red;
        }
    }


    fn insert_node(parent: &Rc<RefCell<TreeNode<T>>>, key: T) {
        let child_to_insert = {
            let mut parent_borrow = parent.borrow_mut();

            match key.cmp(&parent_borrow.key) {
                Ordering::Less => {
                    if parent_borrow.left.is_none() {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent)));
                        parent_borrow.left = Some(new_node);
                        if parent_borrow.color==NodeColor::Black{
                            return;
                        }
                        else if parent_borrow.color==NodeColor::Red {
                            match &parent_borrow.parent {
                                None => {}
                                Some(grandparent) => {
                                   let parent_sibling = grandparent.upgrade().unwrap().borrow().right.clone();
                                    match parent_sibling {
                                        None => {
                                            //perform rotation to convert to line?
                                        }
                                        Some(parent_sibling_value) => {
                                            let parent_sibing_value_borrow = parent_sibling_value.borrow_mut();
                                            if parent_sibing_value_borrow.color ==NodeColor::Black{
                                                //perform rotation
                                            }
                                            else if parent_sibing_value_borrow.color==NodeColor::Red {
                                                //recolor
                                                Self::recolor(parent_borrow);
                                                Self::recolor(parent_sibing_value_borrow);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return; // Exit early after insertion
                    }
                    parent_borrow.left.clone()
                }
                Ordering::Greater | Ordering::Equal => {
                    if parent_borrow.right.is_none() {
                        let new_node = TreeNode::new(key, NodeColor::Red, Some(Rc::downgrade(parent)));
                        parent_borrow.right = Some(new_node);
                        return; // Exit early after insertion
                    }
                    parent_borrow.right.clone()
                }
            }
        };
        if let Some(child) = child_to_insert {
            TreeNode::insert_node(&child, key);
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
                TreeNode::insert_node(root_rc, key)
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
    root.insert(10);
    root.insert(8);
    root.insert(11);
    root.insert(7);
    println!("{:#?}", root);
}
