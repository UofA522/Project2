use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
    None
}
type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;
#[derive(Debug)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl<T> TreeNode<T>{
    fn new(key:T,color:NodeColor)->Self{
        TreeNode{
            color,
            key,
            parent: None,
            left: None,
            right: None,
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
struct RBTree{
    root:RedBlackTree
}

impl RBTree {
    fn new()->Self{
        RBTree{
            root:None
        }
    }
}

fn main() {
    let mut root = RBTree::new();
    println!("{:#?}",root);
}
