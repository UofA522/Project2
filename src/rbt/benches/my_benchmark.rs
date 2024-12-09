use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rbt::RedBlackTreeStructure;

fn insert_and_search_rbt(size:i32){
    let mut rbt = RedBlackTreeStructure::new();
    for i in (0..size)
    {
        rbt.insert(i);
    }
    for i in (0..size/10){
        rbt.find_by_key(i);
    }
}

fn insert(size:i32) -> RedBlackTreeStructure<i32> {
    let mut rbt = RedBlackTreeStructure::new();
    for i in (0..size)
    {
        rbt.insert(i);
    }
    return rbt
}

fn search(numbers:Vec<i32>,rbt: &RedBlackTreeStructure<i32>) {
    for i in numbers {
        rbt.find_by_key(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {

    for i in vec![10000,40000,70000,100000,130000] {
        let rbt = insert(i);
        c.bench_function(format!("rbt_insert_{}",i).as_str(), |b| b.iter(|| insert(black_box(i))));
        c.bench_function(format!("rbt_search_{}",i).as_str(), |b| b.iter(|| search(black_box((0..i/10).collect()),black_box(&rbt))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);