use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use avl::AVLTreeStructure;

fn insert_and_search_avl(size:i32){
    let mut new_avl_tree = AVLTreeStructure::new();
    for i in (0..size)
    {
        new_avl_tree.insert(i);
    }
    for i in (0..size/10){
        new_avl_tree.find_by_key(i);
    }
}

fn insert(size:i32) -> AVLTreeStructure<i32> {
    let mut avl = AVLTreeStructure::new();
    for i in (0..size)
    {
        avl.insert(i);
    }
    return avl
}

fn search(numbers:Vec<i32>,avl: &AVLTreeStructure<i32>) {
    for i in numbers {
        avl.find_by_key(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {

    for i in vec![10000,40000,70000,100000,130000] {
        let avl = insert(i);
        c.bench_function(format!("avl_insert_{}",i).as_str(), |b| b.iter(|| insert(black_box(i))));
        c.bench_function(format!("avl_search_{}",i).as_str(), |b| b.iter(|| search(black_box((0..i/10).collect()),black_box(&avl))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);