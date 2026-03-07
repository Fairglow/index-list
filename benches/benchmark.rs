/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use criterion::{criterion_group, criterion_main, Criterion};
use rand::RngExt;
use slab::Slab;
use std::collections::{LinkedList, vec_deque::VecDeque};
use std::hint::black_box;
use index_list::{Index, IndexList, ListIndex};

fn indexlist_head(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.insert_first(i); });
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += list.remove_first().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn vec_head(n: u32) {
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

fn indexlist_body(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).for_each(|i| {
        list.insert_last(i);
    });
    (0..n as usize).for_each(|i| {
        let val = 0;
        let ndx = list.insert_before(ListIndex::from(i), val);
        let got = list.remove(ndx).unwrap();
        assert_eq!(got, val);
    })
}

fn vec_body(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    (1..=n).for_each(|i| vec.push(i));
    (0..n as usize).for_each(|i| {
        let val = 0;
        vec.insert(i, val);
        let got = vec.remove(i);
        assert_eq!(got, val);
    })
}

fn vecdeque_body(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    (1..=n).for_each(|i| vec.push_back(i));
    (0..n as usize).for_each(|i| {
        let val = 0;
        vec.insert(i, val);
        let got = vec.remove(i).unwrap();
        assert_eq!(got, val);
    })
}

fn indexlist_walk(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.insert_first(i); });
    let mut accum: u64 = 0;
    let mut index = list.first_index();
    while index.is_some() {
        accum += *list.get(index).unwrap() as u64;
        index = list.next_index(index);
    };
    assert_eq!(accum, 52433920);
    index = list.last_index();
    while index.is_some() {
        accum -= *list.get(index).unwrap() as u64;
        index = list.prev_index(index);
    };
    assert_eq!(accum, 0);
}

fn indexlist_iter(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.insert_first(i); });
    let mut accum: u64 = 0;
    list.iter().for_each(|i| { accum += *i as u64; });
    assert_eq!(accum, 52433920);
    list.iter().rev().for_each(|i| { accum -= *i as u64; });
    assert_eq!(accum, 0);
}

fn linkedlist_iter(n: u32) {
    let mut list = LinkedList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.push_front(i); });
    let mut accum: u64 = 0;
    list.iter().for_each(|i| { accum += *i as u64; });
    assert_eq!(accum, 52433920);
    list.iter().rev().for_each(|i| { accum -= *i as u64; });
    assert_eq!(accum, 0);
}

fn indexlist_tail(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).for_each(|i| { list.insert_last(i); });
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += list.remove_last().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn vec_tail(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    (1..=n).for_each(|i| vec.push(i));
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += vec.pop().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn vecdeque_tail(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    (1..=n).for_each(|i| vec.push_back(i));
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += vec.pop_back().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn linkedlist_tail(n: u32) {
    let mut list = LinkedList::<u32>::new();
    (1..=n).for_each(|i| { list.push_back(i); });
    let mut accum: u64 = 0;
    (1..=n).for_each(|_| { accum += list.pop_back().unwrap() as u64; });
    assert_eq!(accum, 52433920);
}

fn indexlist_random(n: u32) {
    let mut list = IndexList::<u32>::new();
    list.insert_first(0);
    let mut rng = rand::rng();
    (0..n).for_each(|_| {
        let len = list.len();
        let index = Index::from(rng.random_range(0..len));
        let val = 0;
        let new_index = list.insert_before(index, val);
        let got = list.remove(new_index).unwrap();
        assert_eq!(got, val);
    });
}

fn vec_random(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    vec.push(0);
    let mut rng = rand::rng();
    (0..n).for_each(|_| {
        let len = vec.len();
        let index = rng.random_range(0..len);
        let val = 0;
        vec.insert(index, val);
        let got = vec.remove(index);
        assert_eq!(got, val);
    });
}

fn vecdeque_random(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    vec.push_front(0);
    let mut rng = rand::rng();
    (0..n).for_each(|_| {
        let len = vec.len();
        let index = rng.random_range(0..len);
        let val = 0;
        vec.insert(index, val);
        let got = vec.remove(index).unwrap();
        assert_eq!(got, val);
    });
}

fn slab_random(n: u32) {
    let mut slab = Slab::<u32>::new();
    slab.insert(0);
    (0..n).for_each(|_| {
        let val = 0;
        let key = slab.insert(val);
        let got = slab.remove(key);
        assert_eq!(got, val);
    });
}

#[cfg(feature = "iter_mut")]
fn indexlist_iter_mut(n: u32) {
    let mut list = IndexList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.insert_first(i); });
    list.iter_mut().for_each(|i| { *i += 1; });
}

#[cfg(feature = "iter_mut")]
fn vec_iter_mut(n: u32) {
    let mut vec: Vec<u32> = Vec::new();
    (1..=n).rev().for_each(|i| vec.insert(0, i));
    vec.iter_mut().for_each(|i| { *i += 1; });
}

#[cfg(feature = "iter_mut")]
fn vecdeque_iter_mut(n: u32) {
    let mut vec: VecDeque<u32> = VecDeque::new();
    (1..=n).rev().for_each(|i| vec.push_front(i));
    vec.iter_mut().for_each(|i| { *i += 1; });
}

#[cfg(feature = "iter_mut")]
fn linkedlist_iter_mut(n: u32) {
    let mut list = LinkedList::<u32>::new();
    (1..=n).rev().for_each(|i| { list.push_front(i); });
    list.iter_mut().for_each(|i| { *i += 1; });
}


fn criterion_benchmark(c: &mut Criterion) {
    let count = 10 * 1024;
    c.bench_function("indexlist-head", |b| b.iter(||
        indexlist_head(black_box(count))));
    c.bench_function("vec-head", |b| b.iter(||
        vec_head(black_box(count))));
    c.bench_function("vecdeque-head", |b| b.iter(||
        vecdeque_head(black_box(count))));
    c.bench_function("linkedlist-head", |b| b.iter(||
        linkedlist_head(black_box(count))));
    c.bench_function("indexlist-body", |b| b.iter(||
        indexlist_body(black_box(count))));
    c.bench_function("vec-body", |b| b.iter(||
        vec_body(black_box(count))));
    c.bench_function("vecdeque-body", |b| b.iter(||
        vecdeque_body(black_box(count))));
    c.bench_function("indexlist-walk", |b| b.iter(||
        indexlist_walk(black_box(count))));
    c.bench_function("indexlist-iter", |b| b.iter(||
        indexlist_iter(black_box(count))));
    c.bench_function("linkedlist-iter", |b| b.iter(||
        linkedlist_iter(black_box(count))));
    c.bench_function("indexlist-tail", |b| b.iter(||
        indexlist_tail(black_box(count))));
    c.bench_function("vec-tail", |b| b.iter(||
        vec_tail(black_box(count))));
    c.bench_function("vecdeque-tail", |b| b.iter(||
        vecdeque_tail(black_box(count))));
    c.bench_function("linkedlist-tail", |b| b.iter(||
        linkedlist_tail(black_box(count))));
    c.bench_function("indexlist-random", |b| b.iter(||
        indexlist_random(black_box(count))));
    c.bench_function("vec-random", |b| b.iter(||
        vec_random(black_box(count))));
    c.bench_function("vecdeque-random", |b| b.iter(||
        vecdeque_random(black_box(count))));
    c.bench_function("slab-random", |b| b.iter(||
        slab_random(black_box(count))));
    #[cfg(feature = "iter_mut")]
    {
        c.bench_function("indexlist-iter-mut", |b| b.iter(||
            indexlist_iter_mut(black_box(count))));
        c.bench_function("vec-iter-mut", |b| b.iter(||
            vec_iter_mut(black_box(count))));
        c.bench_function("vecdeque-iter-mut", |b| b.iter(||
            vecdeque_iter_mut(black_box(count))));
        c.bench_function("linkedlist-iter-mut", |b| b.iter(||
            linkedlist_iter_mut(black_box(count))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
