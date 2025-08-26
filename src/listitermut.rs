/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The definitions of the ListIterMut type
use std::iter::{DoubleEndedIterator, FusedIterator};

use crate::listindex::ListIndex;
use crate::listnode::ListNode;

/// A double-ended mutating iterator over all the elements in the list. It is fused and
/// can be reversed.
pub struct ListIterMut<'a, T> {
    // This field can't be a slice without violating aliasing rules since we're going to be
    // mutably borrowing elements from it.
    pub(crate) elems: *mut Option<T>,
    pub(crate) nodes: &'a [ListNode],
    pub(crate) start: ListIndex,
    pub(crate) end: ListIndex,
    pub(crate) len: usize,
}

impl<T> ListIterMut<'_, T> {
    #[inline]
    fn set_empty(&mut self) {
        self.start = ListIndex::new();
        self.end = ListIndex::new();
        self.len = 0;
    }
}

impl<'a, T: 'a> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.start.get()?;
        // SAFETY: Indices only ever refer to entries in `elems`, so `elems` is a witness that this
        // add operation won't overflow
        let elem_ptr = unsafe { self.elems.add(idx) };
        if self.start == self.end {
            self.set_empty();
        } else {
            self.start = self.nodes[idx].next;
            self.len -= 1;
        }
        // SAFETY: We rely on the fact that the public API to `IndexList` does not provide a way
        // to construct a `nodes` slice which contains cycles or duplicate
        // indices to ensure that each element is visited at most once (this invariant is thus a
        // prerequisite of memory safety). This means that between
        // all `next` and `next_back` calls, we only return at most one unshared reference to each
        // element.
        Some(unsafe { &mut *elem_ptr }.as_mut().unwrap())
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl<'a, T: 'a> FusedIterator for ListIterMut<'a, T> {}
impl<'a, T: 'a> ExactSizeIterator for ListIterMut<'a, T> {}

impl<'a, T: 'a> DoubleEndedIterator for ListIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let idx = self.end.get()?;
        // SAFETY: Indices only ever refer to entries in `elems`, so `elems` is a witness that this
        // add operation won't overflow
        let elem_ptr = unsafe { self.elems.add(idx) };
        if self.start == self.end {
            self.set_empty();
        } else {
            self.end = self.nodes[idx].prev;
            self.len -= 1;
        }
        // SAFETY: We rely on the fact that the public API to `IndexList` does not provide a way
        // to construct a `nodes` slice which contains cycles or duplicate
        // indices to ensure that each element is visited at most once (this invariant is thus a
        // prerequisite of memory safety). This means that between
        // all `next` and `next_back` calls, we only return at most one unshared reference to each
        // element.
        Some(unsafe { &mut *elem_ptr }.as_mut().unwrap())
    }
}
