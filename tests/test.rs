/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::collections::HashSet;
use std::mem::size_of;

use index_list::{IndexList, ListIndex};
use rand::{seq::SliceRandom, RngExt};

fn debug_print_indexes(list: &IndexList<u64>) {
    let mut index = list.first_index();
    let mut last = ListIndex::from(None);
    print!("[ ");
    while index.is_some() {
        if last.is_some() {
            print!(" >< ");
        }
        print!("{}", index);
        debug_assert_eq!(list.prev_index(index), last);
        last = index;
        index = list.next_index(index);
    }
    println!(" ]");
}
fn get_raw_index(index: &ListIndex) -> u32 {
    index.to_string().parse::<u32>().unwrap_or(0)
}

#[test]
fn test_instantiate() {
    let mut list = IndexList::<u64>::new();
    let null = ListIndex::from(None);
    assert_eq!(size_of::<ListIndex>(), 4);
    assert_eq!(list.len(), 0);
    assert_eq!(list.capacity(), 0);
    assert!(!list.is_index_used(null));
    assert_eq!(list.first_index(), null);
    assert_eq!(list.last_index(), null);
    assert_eq!(list.next_index(null), null);
    assert_eq!(list.prev_index(null), null);
    assert_eq!(list.move_index(null, 0), null);
    assert_eq!(list.get(null), None);
    assert_eq!(list.get_first(), None);
    assert_eq!(list.get_last(), None);
    assert_eq!(list.get_mut(null), None);
    assert_eq!(list.get_mut_first(), None);
    assert_eq!(list.get_mut_last(), None);
    assert_eq!(list.peek_next(null), None);
    assert_eq!(list.peek_prev(null), None);
    assert_eq!(list.remove_first(), None);
    assert_eq!(list.remove_last(), None);
    assert_eq!(list.remove(null), None);
    assert_eq!(list.index_of(0), null);
    assert!(!list.contains(0));
    assert_eq!(list.to_vec(), Vec::<&u64>::new());
    let mut empty_list = IndexList::new();
    list.append(&mut empty_list);
    list.prepend(&mut empty_list);
    list.split(null);
    list.trim_safe();
    list.trim_swap();
}
#[test]
fn basic_insert_remove() {
    let mut list = IndexList::<u64>::new();
    let count = 9;
    (0..count).for_each(|i| {
        let ndx = list.insert_first(i);
        assert!(list.is_index_used(ndx));
    });
    println!("{}", list);
    assert_eq!(list.capacity(), count as usize);
    assert_eq!(list.len(), count as usize);
    list.trim_swap();
    (0..count).rev().for_each(|i| {
        assert_eq!(list.remove_first(), Some(i));
        assert!(!list.is_index_used(ListIndex::from(i as usize)));
        assert_eq!(list.len(), i as usize);
    });
    assert_eq!(list.remove_first(), None);
    assert_eq!(list.remove_last(), None);
    assert_eq!(list.capacity(), count as usize);
    list.trim_safe();
    assert_eq!(list.capacity(), 0);
}
#[test]
fn test_while_get_mut() {
    let mut strings = "A B C".split_whitespace().map(String::from).collect();
    let mut list: IndexList<String> = IndexList::from(&mut strings);
    let mut index = list.first_index();
    while index.is_some() {
        let elem = list.get_mut(index).unwrap();
        *elem = elem.to_string().to_lowercase();
        index = list.next_index(index);
    }
    assert_eq!(list.to_string(), "[a >< b >< c]");
}
#[test]
fn test_append() {
    let mut list = IndexList::from(&mut vec!["A", "B", "C"]);
    let mut other = IndexList::from(&mut vec!["D", "E", "F"]);
    list.append(&mut other);
    assert_eq!(list.len(), 6);
    assert_eq!(list.capacity(), 6);
    let index = list.move_index(list.first_index(), 3);
    assert_eq!(list.get(index), Some(&"D"));
    if let Some(chr) = list.get_mut(index) {
        *chr = "G";
    }
    assert_eq!(list.get(index), Some(&"G"));
    let parts: Vec<&str> = list.iter().map(|e| e.as_ref()).collect();
    assert_eq!(parts.join(", "), "A, B, C, G, E, F");
    other = list.split(index);
    assert_eq!(list.to_string(), "[A >< B >< C]");
    assert_eq!(other.to_string(), "[G >< E >< F]");
}
#[test]
fn test_trim_swap() {
    let mut rng = rand::rng();
    let mut list = IndexList::<u64>::new();
    for round in 0..16 {
        debug_print_indexes(&list);
        (0..16).for_each(|i| {
            let num = 16 * round + i;
            let ndx = list.insert_last(num);
            assert_eq!(list.get(ndx), Some(&num));
        });
        debug_print_indexes(&list);
        let mut indexes: Vec<usize> = (0..list.capacity()).collect();
        indexes.shuffle(&mut rng);
        (0..8).for_each(|_| {
            list.remove(ListIndex::from(indexes.pop()));
        });
        debug_print_indexes(&list);
        list.trim_swap();
        assert_eq!(list.capacity(), 8 + 8 * round as usize);
        assert_eq!(list.len(), list.capacity());
    }
}
#[test]
fn test_single_element() {
    let mut list = IndexList::<u64>::new();
    for num in 0..8 {
        match num & 1 {
            0 => list.insert_first(num),
            _ => list.insert_last(num),
        };
        match num & 2 {
            0 => assert_eq!(list.get_first(), Some(&num)),
            _ => assert_eq!(list.get_last(), Some(&num)),
        }
        let val = match num & 4 {
            0 => list.remove_first(),
            _ => list.remove_last(),
        };
        assert_eq!(val, Some(num));
    }
    assert!(list.is_empty());
    assert_eq!(list.capacity(), 1);
    assert_eq!(list.len(), 0);
}
#[test]
fn test_remove_element_twice() {
    let mut list = IndexList::<u64>::new();
    let index = list.insert_first(0);
    let removed1 = list.remove(index);
    assert_eq!(removed1, Some(0));
    let removed2 = list.remove(index);
    assert_eq!(removed2, None);
    assert_eq!(list.len(), 0);
}
#[test]
fn singleton_element_occurs_on_front_xor_back() {
    let list = IndexList::<u64>::from_iter([1]);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next_back(), None);
    let mut iter = list.iter();
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.next(), None);
}
#[test]
fn test_iterator_hint_correct_after_next() {
    let list = IndexList::<u64>::from_iter([1, 2, 3]);
    let mut iter = list.iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.size_hint(), (2, Some(2)));
}
#[test]
fn test_inexhausted_drain_clears_list() {
    let mut list = IndexList::<u64>::from_iter([1, 2, 3]);
    let mut drain = list.drain_iter();
    assert_eq!(drain.next(), Some(1));
    drop(drain);
    assert_eq!(list.len(), 0);
}
#[test]
fn insert_remove_variants() {
    let count = 256;
    let mut rng = rand::rng();
    let mut list = IndexList::<u64>::with_capacity(count);
    let mut numbers: HashSet<u64> = HashSet::with_capacity(count);
    let mut indexes: Vec<u32> = Vec::with_capacity(count);
    for _ in 0..8 {
        for c in 0..count {
            let num = c as u64;
            numbers.insert(num);
            print!("IndexList#{}: insert ", num);
            match c & 3 {
                0 => {
                    let ndx = list.insert_first(num);
                    println!("index {} first", ndx);
                    indexes.push(get_raw_index(&ndx));
                }
                1 => {
                    let that = ListIndex::from(indexes[rng.random_range(0..c)] - 1);
                    print!("before {} ", that);
                    let ndx = list.insert_before(that, num);
                    println!("index {}", ndx);
                    indexes.push(get_raw_index(&ndx));
                }
                2 => {
                    let that = ListIndex::from(indexes[rng.random_range(0..c)] - 1);
                    print!("after {} ", that);
                    let ndx = list.insert_after(that, num);
                    println!("index {} ", ndx);
                    indexes.push(get_raw_index(&ndx));
                }
                _ => {
                    let ndx = list.insert_last(num);
                    println!("index {} last", ndx);
                    indexes.push(get_raw_index(&ndx));
                }
            }
            print!("IndexList: ");
            debug_print_indexes(&list);
        }
        assert_eq!(list.len(), count);
        for c in (1..=count).rev() {
            let ndx = ListIndex::from(indexes.swap_remove(rng.random_range(0..c)) - 1);
            println!("IndexList - remove {}", ndx);
            let num = list.remove(ndx).unwrap();
            //println!("IndexList: {}", list.to_debug_string());
            assert!(numbers.remove(&num));
        }
        assert_eq!(list.capacity(), count);
        assert_eq!(list.len(), 0);
        assert!(numbers.is_empty());
        assert!(indexes.is_empty());
        list.trim_safe();
        assert_eq!(list.capacity(), 0);
    }
}

// ── Clone ──────────────────────────────────────────────────────────

#[test]
fn test_clone_preserves_order_and_independence() {
    let mut list = IndexList::from(&mut vec![1, 2, 3]);
    let cloned = list.clone();
    assert_eq!(list.len(), cloned.len());
    assert_eq!(list.to_string(), cloned.to_string());
    // Mutating original doesn't affect clone
    list.remove_first();
    assert_eq!(list.len(), 2);
    assert_eq!(cloned.len(), 3);
    assert_eq!(cloned.get_first(), Some(&1));
}

#[test]
fn test_clone_empty() {
    let list = IndexList::<u64>::new();
    let cloned = list.clone();
    assert!(cloned.is_empty());
    assert_eq!(cloned.capacity(), 0);
}

// ── PartialEq / Eq ────────────────────────────────────────────────

#[test]
fn test_eq_same_elements() {
    let a = IndexList::from(&mut vec![1, 2, 3]);
    let b = IndexList::from(&mut vec![1, 2, 3]);
    assert_eq!(a, b);
}

#[test]
fn test_eq_different_elements() {
    let a = IndexList::from(&mut vec![1, 2, 3]);
    let b = IndexList::from(&mut vec![1, 2, 4]);
    assert_ne!(a, b);
}

#[test]
fn test_eq_different_lengths() {
    let a = IndexList::from(&mut vec![1, 2]);
    let b = IndexList::from(&mut vec![1, 2, 3]);
    assert_ne!(a, b);
}

#[test]
fn test_eq_different_order() {
    let a = IndexList::from(&mut vec![1, 2, 3]);
    let b = IndexList::from(&mut vec![3, 2, 1]);
    assert_ne!(a, b);
}

#[test]
fn test_eq_empty_lists() {
    let a = IndexList::<u64>::new();
    let b = IndexList::<u64>::new();
    assert_eq!(a, b);
}

#[test]
fn test_eq_after_clone() {
    let list = IndexList::from(&mut vec![10, 20, 30]);
    let cloned = list.clone();
    assert_eq!(list, cloned);
}

// ── Owned IntoIterator ────────────────────────────────────────────

#[test]
fn test_into_iter_forward() {
    let list = IndexList::from(&mut vec![1, 2, 3]);
    let collected: Vec<u64> = list.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_into_iter_backward() {
    let list = IndexList::from(&mut vec![1, 2, 3]);
    let collected: Vec<u64> = list.into_iter().rev().collect();
    assert_eq!(collected, vec![3, 2, 1]);
}

#[test]
fn test_into_iter_double_ended() {
    let list = IndexList::from(&mut vec![1, 2, 3, 4]);
    let mut it = list.into_iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next_back(), Some(4));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn test_into_iter_size_hint() {
    let list = IndexList::from(&mut vec![10, 20, 30]);
    let mut it = list.into_iter();
    assert_eq!(it.len(), 3);
    it.next();
    assert_eq!(it.len(), 2);
    it.next_back();
    assert_eq!(it.len(), 1);
    it.next();
    assert_eq!(it.len(), 0);
}

#[test]
fn test_into_iter_empty() {
    let list = IndexList::<u64>::new();
    let mut it = list.into_iter();
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
    assert_eq!(it.len(), 0);
}

#[test]
fn test_into_iter_for_loop() {
    let list = IndexList::from(&mut vec![10, 20, 30]);
    let mut sum = 0u64;
    for val in list {
        sum += val;
    }
    assert_eq!(sum, 60);
}

// ── shift_index operations ────────────────────────────────────────

#[test]
fn test_shift_index_before() {
    let mut list = IndexList::from(&mut vec![1, 2, 3, 4]);
    let last = list.last_index();
    let second = list.next_index(list.first_index());
    // Move 4 before 2: [1, 4, 2, 3]
    assert!(list.shift_index_before(last, second));
    assert_eq!(list.to_string(), "[1 >< 4 >< 2 >< 3]");
    assert_eq!(list.get(last), Some(&4));
}

#[test]
fn test_shift_index_after() {
    let mut list = IndexList::from(&mut vec![1, 2, 3, 4]);
    let first = list.first_index();
    let third = list.move_index(first, 2);
    // Move 1 after 3: [2, 3, 1, 4]
    assert!(list.shift_index_after(first, third));
    assert_eq!(list.to_string(), "[2 >< 3 >< 1 >< 4]");
    assert_eq!(list.get(first), Some(&1));
}

#[test]
fn test_shift_index_to_front() {
    let mut list = IndexList::from(&mut vec![1, 2, 3]);
    let last = list.last_index();
    assert!(list.shift_index_to_front(last));
    assert_eq!(list.to_string(), "[3 >< 1 >< 2]");
    assert_eq!(list.first_index(), last);
}

#[test]
fn test_shift_index_to_back() {
    let mut list = IndexList::from(&mut vec![1, 2, 3]);
    let first = list.first_index();
    assert!(list.shift_index_to_back(first));
    assert_eq!(list.to_string(), "[2 >< 3 >< 1]");
    assert_eq!(list.last_index(), first);
}

#[test]
fn test_shift_index_invalid() {
    let mut list = IndexList::from(&mut vec![1, 2, 3]);
    let first = list.first_index();
    let null = ListIndex::from(None);
    // Same index
    assert!(!list.shift_index_before(first, first));
    // Invalid index
    assert!(!list.shift_index_before(null, first));
    assert!(!list.shift_index_after(first, null));
    assert!(!list.shift_index_to_front(null));
    assert!(!list.shift_index_to_back(null));
}

// ── swap_index ────────────────────────────────────────────────────

#[test]
fn test_swap_index() {
    let mut list = IndexList::from(&mut vec![10, 20, 30]);
    let first = list.first_index();
    let last = list.last_index();
    list.swap_index(first, last);
    assert_eq!(list.get(first), Some(&30));
    assert_eq!(list.get(last), Some(&10));
    assert_eq!(list.to_string(), "[30 >< 20 >< 10]");
}

// ── prepend ───────────────────────────────────────────────────────

#[test]
fn test_prepend() {
    let mut list = IndexList::from(&mut vec![4, 5, 6]);
    let mut other = IndexList::from(&mut vec![1, 2, 3]);
    list.prepend(&mut other);
    assert!(other.is_empty());
    assert_eq!(list.len(), 6);
    assert_eq!(list.to_string(), "[1 >< 2 >< 3 >< 4 >< 5 >< 6]");
}

// ── Extend ────────────────────────────────────────────────────────

#[test]
fn test_extend() {
    let mut list = IndexList::from(&mut vec![1, 2]);
    list.extend(vec![3, 4, 5]);
    assert_eq!(list.len(), 5);
    assert_eq!(list.to_string(), "[1 >< 2 >< 3 >< 4 >< 5]");
}

// ── FromIterator ──────────────────────────────────────────────────

#[test]
fn test_from_iterator() {
    let list: IndexList<u64> = (1..=5).collect();
    assert_eq!(list.len(), 5);
    assert_eq!(list.to_string(), "[1 >< 2 >< 3 >< 4 >< 5]");
}

// ── From single element ───────────────────────────────────────────

#[test]
fn test_from_single_element() {
    let list: IndexList<u64> = 42u64.into();
    assert_eq!(list.len(), 1);
    assert_eq!(list.get_first(), Some(&42));
}

// ── Display ───────────────────────────────────────────────────────

#[test]
fn test_display_empty() {
    let list = IndexList::<u64>::new();
    assert_eq!(list.to_string(), "[]");
}

#[test]
fn test_display_single() {
    let list: IndexList<u64> = 99u64.into();
    assert_eq!(list.to_string(), "[99]");
}

#[test]
fn test_display_multiple() {
    let list = IndexList::from(&mut vec![1, 2, 3]);
    assert_eq!(list.to_string(), "[1 >< 2 >< 3]");
}

// ── contains / index_of ───────────────────────────────────────────

#[test]
fn test_contains_and_index_of() {
    let list = IndexList::from(&mut vec![10, 20, 30]);
    assert!(list.contains(20));
    assert!(!list.contains(99));
    let idx = list.index_of(30);
    assert!(idx.is_some());
    assert_eq!(list.get(idx), Some(&30));
    let missing = list.index_of(99);
    assert!(missing.is_none());
}

// ── with_capacity ─────────────────────────────────────────────────

#[test]
fn test_with_capacity() {
    let list = IndexList::<u64>::with_capacity(100);
    assert!(list.is_empty());
    assert_eq!(list.capacity(), 0);
    assert_eq!(list.len(), 0);
}

// ── drain_iter double-ended and size_hint ─────────────────────────

#[test]
fn test_drain_iter_double_ended() {
    let mut list = IndexList::from(&mut vec![1, 2, 3, 4]);
    let mut drain = list.drain_iter();
    assert_eq!(drain.len(), 4);
    assert_eq!(drain.next(), Some(1));
    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next_back(), Some(4));
    assert_eq!(drain.len(), 2);
    assert_eq!(drain.next(), Some(2));
    assert_eq!(drain.next_back(), Some(3));
    assert_eq!(drain.next(), None);
    assert_eq!(drain.next_back(), None);
    assert_eq!(drain.len(), 0);
}

// ── iter rev ──────────────────────────────────────────────────────

#[test]
fn test_iter_rev() {
    let list = IndexList::from(&mut vec![1, 2, 3]);
    let reversed: Vec<&u64> = list.iter().rev().collect();
    assert_eq!(reversed, vec![&3, &2, &1]);
}

// ── for &list (IntoIterator for &IndexList) ───────────────────────

#[test]
fn test_ref_into_iterator() {
    let list = IndexList::from(&mut vec![10, 20, 30]);
    let mut sum = 0u64;
    for val in &list {
        sum += val;
    }
    assert_eq!(sum, 60);
    assert_eq!(list.len(), 3);
}

// ── ListIndex display ─────────────────────────────────────────────

#[test]
fn test_list_index_display() {
    let null = ListIndex::from(None);
    assert_eq!(null.to_string(), "|");
    let list = IndexList::from(&mut vec![42u64]);
    let idx = list.first_index();
    assert!(idx.is_some());
    assert_eq!(idx.to_string(), "1");
}

// ── move_index ────────────────────────────────────────────────────

#[test]
fn test_move_index_forward_and_back() {
    let list = IndexList::from(&mut vec!["A", "B", "C", "D", "E"]);
    let idx = list.first_index();
    let moved = list.move_index(idx, 4);
    assert_eq!(list.get(moved), Some(&"E"));
    let back = list.move_index(moved, -3);
    assert_eq!(list.get(back), Some(&"B"));
    let past_end = list.move_index(moved, 1);
    assert!(past_end.is_none());
}

// ── peek_next / peek_prev ─────────────────────────────────────────

#[test]
fn test_peek_next_prev() {
    let list = IndexList::from(&mut vec![1, 2, 3]);
    let first = list.first_index();
    let last = list.last_index();
    assert_eq!(list.peek_next(first), Some(&2));
    assert_eq!(list.peek_prev(last), Some(&2));
    assert_eq!(list.peek_prev(first), None);
    assert_eq!(list.peek_next(last), None);
}
