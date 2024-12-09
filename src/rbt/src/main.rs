use rbt::{RedBlackTreeStructure};
use common::Dotfile;

fn main() {
    let mut rb_tree = RedBlackTreeStructure::new();
    rb_tree.insert(10);
    rb_tree.insert(20);
    rb_tree.insert(30);
    rb_tree.insert(15);
    rb_tree.insert(25);
    rb_tree.insert(31);
    rb_tree.insert(5);
    rb_tree.insert(6);
    rb_tree.insert(1);
    rb_tree.insert(43);
    println!("Searching 5");
    println!("{:#?}",rb_tree.find_by_key(5));
    let count = RedBlackTreeStructure::<u32>::number_of_leaves(&rb_tree.root);
    let height = RedBlackTreeStructure::height_of_tree(&rb_tree.root);
    println!("{:#?}", rb_tree.root);
    println!("Leaf Count:{}", count);
    println!("Height:{}", height);
    println!("Tree traversal");
    RedBlackTreeStructure::in_order_traversal(&rb_tree.root);
    println!("Is tree Empty:{}", rb_tree.tree_is_empty());
    let mut dot_file = Dotfile::new("./rbt.dot");
    rb_tree.draw_tree(&mut dot_file);
    dot_file.write_file();
    rb_tree.delete(5);
    rb_tree.delete(30);
    rb_tree.delete(10);
    rb_tree.delete(43);
    let mut dot_file = Dotfile::new("./rbt_after_delete.dot");
    rb_tree.draw_tree(&mut dot_file);
    dot_file.write_file();
    println!("{:#?}", rb_tree.root);
    RedBlackTreeStructure::in_order_traversal(&rb_tree.root);
}
