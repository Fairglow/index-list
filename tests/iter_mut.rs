#![cfg(feature = "iter_mut")]
/*
 * Tests for the mutating iterator (iter_mut) feature.
 */
use index_list::IndexList;

#[test]
fn iter_mut_basic_mutation_and_order() {
    let mut list = IndexList::from(&mut vec![1, 2, 3, 4]);
    {
        let mut it = list.iter_mut();
        assert_eq!(it.size_hint(), (4, Some(4)));
        *it.next().unwrap() *= 10; // 1 -> 10
        *it.next_back().unwrap() *= 10; // 4 -> 40
        *it.next().unwrap() *= 10; // 2 -> 20
        assert_eq!(it.size_hint(), (1, Some(1)));
        *it.next_back().unwrap() *= 10; // 3 -> 30
        assert_eq!(it.next(), None);
        assert_eq!(it.next_back(), None);
    }
    // After iterator is dropped we can drain to verify final order/content
    let collected: Vec<_> = list.drain_iter().collect();
    assert_eq!(collected, vec![10, 20, 30, 40]);
}

#[test]
fn iter_mut_empty() {
    let mut list = IndexList::<u32>::new();
    let mut it = list.iter_mut();
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
    assert_eq!(it.size_hint(), (0, Some(0)));
}

#[test]
fn iter_mut_fused() {
    let mut list = IndexList::from(&mut vec![5, 6]);
    let mut it = list.iter_mut();
    assert!(it.next().is_some());
    assert!(it.next().is_some());
    assert_eq!(it.next(), None); // exhausted
    assert_eq!(it.next_back(), None); // still None after exhaustion (fused)
}
