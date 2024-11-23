use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
    None
}
type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T>= Option<Tree<T>>;
#[derive(Debug)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

impl<T:Ord> TreeNode<T>{
    fn new(key:T, color:NodeColor) -> Rc<RefCell<TreeNode<T>>> {
        Rc::new(RefCell::new(TreeNode{
            color,
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }
    fn insert_node(&mut self,key:T){
        match key.cmp(&self.key) {
            Ordering::Less => {
                match &self.left {
                    None => {
                        self.left = Some(TreeNode::new(key,NodeColor::Red))
                    }
                    Some(left) => {
                        left.borrow_mut().insert_node(key)
                    }
                }
            }
            Ordering::Equal | Ordering::Greater  => {
                match &self.right {
                    None => {
                        self.right = Some(TreeNode::new(key,NodeColor::Red))
                    }
                    Some(right) => {
                        right.borrow_mut().insert_node(key)
                    }
                }
            }
        }
    }
}

trait BasicFunction<T>{
    fn insert(&mut self,key:T);
    fn delete(&mut self,key:T);
    fn number_of_leaves(&self)->u32;
    fn height_of_tree(&self)->u32;
    fn inorder_traversal(&self);
    fn is_tree_empty(&self)->bool;
    fn print_tree(&self);

}

#[derive(Debug)]
struct RBTree<T>{
    root:RedBlackTree<T>
}

impl<T:Ord> RBTree<T> {
    fn new()->Self{
        RBTree{
            root:None
        }
    }
}


impl<T:Ord + Clone + std::fmt::Debug> BasicFunction<T> for RBTree<T> {
    fn insert(&mut self, key: T) {

        match &self.root {
            None => {
                self.root = Some(TreeNode::new(key.clone(), NodeColor::Black));;
            }
            Some(root_val) => {
                root_val.borrow_mut().insert_node(key)
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
        todo!()
    }

    fn print_tree(&self) {
        todo!()
    }
}

fn main() {
    let mut root = RBTree::new();
    root.insert(5);
    root.insert(1);
    println!("{:#?}",root);
}
