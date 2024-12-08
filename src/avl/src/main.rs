use avl::AVLTreeStructure;
use common::Dotfile;

fn main() {
    let mut avl = AVLTreeStructure::new();
    avl.insert(10);
    avl.insert(20);
    avl.insert(5);
    avl.insert(100);
    avl.insert(15);
    avl.insert(1);
    avl.insert(25);
    avl.insert(26);
    avl.insert(34);
    avl.insert(30);
    println!("{:#?}", avl);
    let mut dot_file = Dotfile::new("./avl.dot");
    avl.draw_tree(&mut dot_file);
    dot_file.write_file();
    avl.delete(5);
    avl.delete(25);
    println!("After delete");
    let mut dot_file = Dotfile::new("./avl_after_delete.dot");
    avl.draw_tree(&mut dot_file);
    dot_file.write_file();
    println!("{:#?}", avl);
}
