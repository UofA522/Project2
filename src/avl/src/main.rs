use avl::AVLTreeStructure;

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
    avl.delete(5);
    avl.delete(25);
    println!("After delete");
    println!("{:#?}", avl);
}
