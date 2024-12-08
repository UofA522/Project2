use avl::AVLTreeStructure;

fn main() {
    let mut avl = AVLTreeStructure::new();
    avl.insert(10);
    avl.insert(20);
    avl.insert(5);
    avl.insert(15);
    avl.insert(25);
    avl.insert(30);
    println!("{:#?}", avl);
}
