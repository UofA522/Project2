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
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl<T> TreeNode<T>{
    fn new(key:T)->Self{
        TreeNode{
            color: NodeColor::Black,
            key,
            parent: None,
            left: None,
            right: None,
        }
    }
}

trait BasicFunction{
    fn insert(&self);
    fn delete(&self);
    fn number_of_leaves(&self)->u32;
    fn height_of_tree(&self)->u32;
    fn inorder_traversal(&self);
    fn is_tree_empty(&self)->bool;
    fn print_tree(&self);

}
impl<T> BasicFunction for TreeNode<T>{
    fn insert(&self) {
        todo!()
    }

    fn delete(&self) {
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

    fn is_tree_empty(&self) ->bool {
        todo!()
    }

    fn print_tree(&self) {
        todo!()
    }
}


fn main() {
    println!("Hello, world!");
}
