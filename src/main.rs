use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
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
    color: NodeColor,
    key: T,
    parent: Option<WeakTree<T>>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

struct Dotfile {
    filename: String,
    nodes: Vec<DotNode>,
    edges: Vec<DotEdge>,
}
struct DotNode {
    idx: usize,
    label: String,
    color: String,
    font_color: String,
}

struct DotEdge {
    src_id: usize,
    dest_id: usize,
}


impl Dotfile {
    fn new(filename: &str) -> Self {
        Dotfile {
            filename: filename.to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    fn add_node(&mut self, key: &str, node_color: NodeColor) -> usize {
        let current_len = self.nodes.len();
        let color = match node_color {
            NodeColor::Red => { "red" }
            NodeColor::Black => { "black" }
        };
        self.nodes.push(DotNode {
            idx: current_len,
            label: key.to_string(),
            color: color.to_string(),
            font_color: "white".to_string(),
        });
        current_len
    }

    fn add_edge(&mut self, key1: usize, key2: usize) {
        self.edges.push(DotEdge {
            src_id: key1,
            dest_id: key2,
        })
    }

    fn write_file(&self) {
        let mut dot_string = String::new();
        dot_string.push_str("graph {\n");

        for node in &self.nodes {
            dot_string.push_str(&format!("\t {} [label=\"{}\", color={}, style=filled, fontcolor={}];\n", node.idx, node.label, node.color, node.font_color))
        }
        for edge in &self.edges {
            dot_string.push_str(&format!("\t {} -- {};\n", edge.src_id, edge.dest_id))
        }
        dot_string.push_str("}\n");
        let mut dot_file = File::create(&self.filename).expect("Error while Creating file");
        dot_file.write_all(dot_string.as_bytes()).expect("W")
    }
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

    fn draw_node(node: &RedBlackTree<T>, file: &mut Dotfile, mut parent_node_idx: Option<usize>) {
        if let Some(root) = node {
            let root_node = match parent_node_idx {
                None => { file.add_node(format!("{:?}", root.clone().borrow().key.clone()).as_str(), root.clone().borrow().color.clone()) }
                Some(parent_val) => {
                    parent_val
                }
            };

            if let Some(left) = root.clone().borrow().left.clone() {
                let left_node = file.add_node(format!("{:?}", left.borrow().key.clone()).as_str(), left.borrow().color.clone());
                file.add_edge(root_node, left_node);
                Self::draw_node(&root.clone().borrow().left.clone(), file, Some(left_node));
            } else {
                let left_node = file.add_node("None", NodeColor::Black);
                file.add_edge(root_node, left_node);
            }
            if let Some(right) = root.clone().borrow().right.clone() {
                let right_node = file.add_node(format!("{:?}", right.borrow().key.clone()).as_str(), right.borrow().color.clone());
                file.add_edge(root_node, right_node);
                Self::draw_node(&root.clone().borrow().right.clone(), file, Some(right_node));
            } else {
                let right_node = file.add_node("None", NodeColor::Black);
                file.add_edge(root_node, right_node);
            }
        }
    }
}
#[derive(Debug)]
struct RedBlackTreeStructure<T> {
    root: RedBlackTree<T>,
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
    fn number_of_leaves(root: &RedBlackTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }
        if root.clone().unwrap().borrow().left.is_none() && root.clone().unwrap().borrow().left.is_none() {
            return 1;
        }
        return RedBlackTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().left.clone()) + RedBlackTreeStructure::<T>::number_of_leaves(&root.clone().unwrap().borrow().right.clone());
    }

    fn height_of_tree(root: &RedBlackTree<T>) -> u32 {
        if root.is_none() {
            return 0;
        }

        let left_height = Self::height_of_tree(&root.clone().unwrap().borrow().left.clone());
        let right_height = Self::height_of_tree(&root.clone().unwrap().borrow().right.clone());
        std::cmp::max(left_height, right_height) + 1
    }

    fn in_order_traversal(root: &RedBlackTree<T>) {
        if root.is_some() {
            Self::in_order_traversal(&root.clone().unwrap().borrow().left.clone());
            println!("{}", root.clone().unwrap().borrow().key);
            Self::in_order_traversal(&root.clone().unwrap().borrow().right.clone());
        }
    }

    fn tree_is_empty(&self) -> bool {
        if self.root.is_none() {
            return true;
        }
        false
    }

    fn draw_tree(&self, file: &mut Dotfile) {
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
        while let Some(parent) = node.clone().borrow().parent.clone().and_then(|p| p.upgrade()) {
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
                        node = parent.clone();
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
            }
        }

        node.borrow_mut().color = NodeColor::Black;
    }

    fn delete_node(&mut self, node: Tree<T>) {
        let mut to_fix = node.clone();

        if node.borrow().left.is_some() && node.borrow().right.is_some() {
            // Find in-order successor
            let successor = self.find_min(node.borrow().left.clone().unwrap());
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
    fn delete(&mut self, key: T) {
        if let Some(node) = self.find_node(key) {
            self.delete_node(node)
        }
    }

    fn fix_delete_double_black(&mut self, node: Tree<T>) {
        if node.borrow().parent.is_none() {
            return;
        }

        let parent = node.borrow().parent.clone().and_then(|p| p.upgrade()).unwrap();
        let is_left = Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap());
        let sibling = if is_left {
            parent.borrow().right.clone()
        } else {
            parent.borrow().left.clone()
        };

        // Handle sibling cases as in `fix_delete`
        if let Some(sibling_node) = sibling {
            // Implement double-black handling logic
        }
    }
    fn find_mininmum_node(&self, node: Tree<T>) -> Tree<T> {
        let mut current = node.clone();
        while let Some(left) = current.clone().borrow().left.clone() {
            current = left.clone()
        }
        current
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
    rb_tree.insert(43);
    let count = RedBlackTreeStructure::<u32>::number_of_leaves(&rb_tree.root);
    let height = RedBlackTreeStructure::height_of_tree(&rb_tree.root);
    println!("{:#?}", rb_tree.root);
    println!("Leaf Count:{}", count);
    println!("Height:{}", height);
    println!("Tree traversal");
    println!("Is tree Empty:{}", rb_tree.tree_is_empty());
    RedBlackTreeStructure::in_order_traversal(&rb_tree.root);
    let mut dot_file = Dotfile::new("./rbt.dot");
    rb_tree.draw_tree(&mut dot_file);
    dot_file.write_file();
    rb_tree.delete(5);
    rb_tree.delete(30);
    rb_tree.delete(10);
    let mut dot_file = Dotfile::new("./rbt_after_delete.dot");
    rb_tree.draw_tree(&mut dot_file);
    dot_file.write_file();
    println!("{:#?}", rb_tree.root);
}
