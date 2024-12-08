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

fn criterion_benchmark(c: &mut Criterion) {

    for i in vec![10000,40000,70000,100000,130000] {
        c.bench_function(format!("avl_insert_and_search{}",i).as_str(), |b| b.iter(|| insert_and_search_avl(black_box(i))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);