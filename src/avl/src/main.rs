use avl::AVLTreeStructure;

fn main() {
    let mut avl_tree = AVLTreeStructure::new();

    avl_tree.insert(10);
    avl_tree.insert(20);
    avl_tree.insert(30);
    avl_tree.insert(15);

    // The AVL tree should be balanced after these insertions.
}
