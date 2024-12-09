use common::Dotfile;
use rbt::{RedBlackTreeStructure};
use avl::AVLTreeStructure;
use std::process::Command;

use std::env;
use clap::Parser;
use std::io;
// #[derive(Debug, Clone)]
// enum TreeType {
//     RedBlackTree,
//     AVL_Tree
// }

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "CLI Program to choose between RBT and AVL Trees",
    long_about = "CLI Program to choose between RBT and AVL Trees"
)]
struct Args {
    /// Ticker name of a stock.
    #[arg(short, long)]
    tree_type: String,

}

fn main() {
    let args = Args::parse();    
    
    match args.tree_type.as_str() {
        "RedBlackTree" => {
            println!("RBT chosen");
            let mut rb_tree = RedBlackTreeStructure::new();
            while true {
                println!("Enter 1 for Insertion operation");
                println!("Enter 2 for Deletion operation");
                println!("Enter 3 for Number of leaves in the Tree");
                println!("Enter 4 for Height of Tree");
                println!("Enter 5 for In-order traversal of the Tree");
                println!("Enter 6 to check if the tree is empty");
                println!("Enter 7 to print the tree showing its structure");
                
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).expect("Failed to read line");
                let choice: i32 = input_line.trim().parse().expect("Input not an integer");
                
                
                match choice {
                    1 => {
                        println!("Enter a comma-separated list of numbers:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");
    
                        let numbers: Vec<i32> = input.trim().split(',').map(|s| s.parse().expect("Numbers not entered.")).collect();
    
                        println!("Parsed numbers: {:?}", numbers);
    
                        for i in numbers {
                            rb_tree.insert(i);
                            println!("{} was inserted in the tree", i);
                        }
                    }

                    2 => {
                        println!("Enter a number to delete:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");
    
                        let num_to_delete: i32 = input.trim().parse().expect("Input not an integer");
                        // println!("{num_to_delete}");
                        // println!("{:#?}", rb_tree);
                        rb_tree.delete(num_to_delete);
                    }
    
                    3 => {
                        let count = RedBlackTreeStructure::<i32>::number_of_leaves(&rb_tree.root);
                        println!("Number of leaves in this Red Black Tree is {}", count);
                    }
    
                    4 => {
                        let height = RedBlackTreeStructure::height_of_tree(&rb_tree.root);
                        println!("The height of this AVL Tree is {}", height);
                    }
    
                    5 => {
                        RedBlackTreeStructure::in_order_traversal(&rb_tree.root);
                    }
    
                    6 => {
                        println!("Is tree Empty:{}", rb_tree.tree_is_empty());
                    }
                    
                    7 => {
                        let mut dot_file = Dotfile::new("./rbt.dot");
                        rb_tree.draw_tree(&mut dot_file);
                        dot_file.write_file();
                        let output = Command::new("dot")
                        .arg("-Tpng")
                        .arg("rbt.dot")
                        .arg("-o")
                        .arg("rbt.png")
                        .output()
                        .expect("Dot hasn't been added to path or hasn't been installed, visit this link to install Graphviz: https://graphviz.org/download/");
                
                        println!("Status: {}", output.status);
                        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                        println!("Successfully generated the png file. File path: ./rbt.png");
                    } 

                    0 => {
                        break;
                    }
    
                    _ => {
                        println!("Please enter a valid choice");
                    }
                }
            }

        }
        "AVL_Tree" => {
            println!("AVL chosen");
            let mut avl = AVLTreeStructure::new();
            
            while true {
                println!("Enter 1 for Insertion operation");
                println!("Enter 2 for Deletion operation");
                println!("Enter 3 for Number of leaves in the Tree");
                println!("Enter 4 for Height of Tree");
                println!("Enter 5 for In-order traversal of the Tree");
                println!("Enter 6 to check if the tree is empty");
                println!("Enter 7 to print the tree showing its structure");

                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).expect("Failed to read line");
                let choice: i32 = input_line.trim().parse().expect("Input not an integer");


                match choice {
                    1 => {
                        println!("Enter a comma-separated list of numbers:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");

                        let numbers: Vec<i32> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();

                        println!("Parsed numbers: {:?}", numbers);

                        for i in numbers {
                            avl.insert(i);
                            println!("{} was inserted in the tree", i);
                        }
                    }
                    2 => {
                        println!("Enter a number to delete:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");

                        let num_to_delete: i32 = input.trim().parse().expect("Input not an integer");
                        avl.delete(num_to_delete);
                    }

                    3 => {
                        let number_of_leaves = AVLTreeStructure::number_of_leaves(&avl.root);
                        println!("Number of leaves in this AVL Tree is {}", number_of_leaves);
                    }

                    4 => {
                        let mut height_of_tree = AVLTreeStructure::height_of_tree(&avl.root);
                        println!("The height of this AVL Tree is {}", height_of_tree);
                    }

                    5 => {
                        AVLTreeStructure::in_order_traversal(&avl.root);
                    }

                    6 => {
                        println!("Is tree empty: {}",avl.tree_is_empty());
                    }
                    
                    7 => {
                        let mut dot_file = Dotfile::new("./avl.dot");
                        avl.draw_tree(&mut dot_file);
                        dot_file.write_file();
                        let output = Command::new("dot")
                        .arg("-Tpng")
                        .arg("avl.dot")
                        .arg("-o")
                        .arg("avl.png")
                        .output()
                        .expect("Dot hasn't been added to path or hasn't been installed, visit this link to install Graphviz: https://graphviz.org/download/");
                
                        println!("Status: {}", output.status);
                        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                        println!("Successfully generated the png file. File path: ./avl.png");
                    }

                    0 => {
                        break;
                    }

                    _ => {
                        println!("Please enter a valid choice");
                    }
                }
            }
        }

        _ => {
            println!("Invalid input");
        }
    }
}
