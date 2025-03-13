/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The defintions of the ListIter type
use std::iter::{DoubleEndedIterator, FusedIterator};

use crate::{listindex::ListIndex, IndexList};

/// A double-ended iterator over all the elements in the list. It is fused and
/// can be reversed.
pub struct ListIter<'a, T> {
    pub(crate) list: &'a IndexList<T>,
    pub(crate) start: ListIndex,
    pub(crate) end: ListIndex,
    pub(crate) len: usize,
}

impl<T> ListIter<'_, T> {
    #[inline]
    fn set_empty(&mut self) {
        self.start = ListIndex::new();
        self.end = ListIndex::new();
        self.len = 0;
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.start)?;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.start = self.list.next_index(self.start);
            self.len -= 1;
        }
        Some(item)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl<T> FusedIterator for ListIter<'_, T> {}
impl<T> ExactSizeIterator for ListIter<'_, T> {}

impl<T> DoubleEndedIterator for ListIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.end)?;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.end = self.list.prev_index(self.end);
            self.len -= 1;
        }
        Some(item)
    }
}
