use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::vec_deque::VecDeque;
use std::collections::LinkedList;
use indexlist::*;

fn indexlist_head(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.insert_first(i); });
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += list.remove_first().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn vec_first(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    (1..=n).rev().for_each(|i| vec.insert(0, i));
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += vec.remove(0) as u64; });
    assert_eq!(accum, 52433920);
}

fn vecdeque_head(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    (1..=n).rev().for_each(|i| vec.push_front(i));
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += vec.pop_front().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn linkedlist_head(n: u32) {
    let mut list = LinkedList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.push_front(i); });
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += list.pop_front().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn indexlist_allover(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).for_each(|i| {
        list.insert_last(i);
    });
    (0..n as usize).for_each(|i| {
        let val = 0;
        let ndx = list.insert_before(i, val);
        let got = list.remove_index(ndx).unwrap();
        assert_eq!(got, val);
    })
}

fn vec_allover(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    (1..=n).for_each(|i| vec.push(i));
    (0..n as usize).for_each(|i| {
        let val = 0;
        vec.insert(i, val);
        let got = vec.remove(i);
        assert_eq!(got, val);
    })
}

fn vecdeque_allover(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    (1..=n).for_each(|i| vec.push_back(i));
    (0..n as usize).for_each(|i| {
        let val = 0;
        vec.insert(i, val);
        let got = vec.remove(i).unwrap();
        assert_eq!(got, val);
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    let count = 10 * 1024;
    c.bench_function("indexlist-head", |b| b.iter(||
        indexlist_head(black_box(count))));
    c.bench_function("vec-first", |b| b.iter(||
        vec_first(black_box(count))));
    c.bench_function("vecdeque-head", |b| b.iter(||
        vecdeque_head(black_box(count))));
    c.bench_function("linkedlist-head", |b| b.iter(||
        linkedlist_head(black_box(count))));
    c.bench_function("indexlist-allover", |b| b.iter(||
        indexlist_allover(black_box(count))));
    c.bench_function("vec-allover", |b| b.iter(||
        vec_allover(black_box(count))));
    c.bench_function("vecdeque-allover", |b| b.iter(||
        vecdeque_allover(black_box(count))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
