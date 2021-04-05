use std::fmt;
use std::num::NonZeroU32;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Index(Option<NonZeroU32>);

impl Index {
    #[inline]
    fn new() -> Index {
        Index { 0: None }
    }
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    #[inline]
    fn get(&self) -> Option<usize> {
        Some(self.0?.get() as usize - 1)
    }
    #[inline]
    fn set(mut self, index: Option<usize>) -> Self {
        if let Some(n) = index {
            if let Ok(num) = NonZeroU32::try_from(n as u32 + 1) {
                self.0 = Some(num);
            } else {
                self.0 = None;
            }
        } else {
            self.0 = None;
        }
        self
    }
}

impl From<u32> for Index {
    fn from(index: u32) -> Index {
        Index::new().set(Some(index as usize))
    }
}

impl From<u64> for Index {
    fn from(index: u64) -> Index {
        Index::new().set(Some(index as usize))
    }
}

impl From<usize> for Index {
    fn from(index: usize) -> Index {
        Index::new().set(Some(index))
    }
}

impl From<Option<usize>> for Index {
    fn from(index: Option<usize>) -> Index {
        Index::new().set(index)
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ndx) = self.0 {
            write!(f, "{}", ndx)
        } else {
            write!(f, "|")
        }
    }
}

#[derive(Clone, Debug, Default)]
struct IndexNode {
    next: Index,
    prev: Index,
}

impl IndexNode {
    #[inline]
    pub fn new() -> IndexNode {
        IndexNode { next: Index::new(), prev: Index::new() }
    }
    #[inline]
    pub fn new_next(&mut self, next: Index) -> Index {
        let old_next = self.next;
        self.next = next;
        old_next
    }
    #[inline]
    pub fn new_prev(&mut self, prev: Index) -> Index {
        let old_prev = self.prev;
        self.prev = prev;
        old_prev
    }
}

impl fmt::Display for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}<>{}", self.next, self.prev)
    }
}

#[derive(Clone, Debug, Default)]
struct IndexEnds {
    head: Index,
    tail: Index,
}

impl IndexEnds {
    #[inline]
    fn new() -> Self {
        IndexEnds { head: Index::new(), tail: Index::new() }
    }
    #[inline]
    fn clear(&mut self) {
        self.new_both(Index::new());
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    #[inline]
    fn new_head(&mut self, head: Index) -> Index {
        let old_head = self.head;
        self.head = head;
        old_head
    }
    #[inline]
    fn new_tail(&mut self, tail: Index) -> Index {
        let old_tail = self.tail;
        self.tail = tail;
        old_tail
    }
    #[inline]
    fn new_both(&mut self, both: Index) {
        self.head = both;
        self.tail = both;
    }
}

impl fmt::Display for IndexEnds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}>=<{}", self.head, self.tail)
    }
}

#[derive(Debug)]
pub struct IndexList<T> {
    elems: Vec<Option<T>>,
    nodes: Vec<IndexNode>,
    used: IndexEnds,
    free: IndexEnds,
    size: usize,
}

impl<T> Default for IndexList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IndexList<T> {
    pub fn new() -> Self {
        IndexList {
            elems: Vec::new(),
            nodes: Vec::new(),
            used: IndexEnds::new(),
            free: IndexEnds::new(),
            size: 0,
        }
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.elems.len()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }
    #[inline]
    pub fn clear(&mut self) {
        self.elems.clear();
        self.nodes.clear();
        self.used.clear();
        self.free.clear();
        self.size = 0;
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.used.is_empty()
    }
    #[inline]
    pub fn is_index_used(&self, index: Index) -> bool {
        if index.is_none() { return false; }
        if let Some(data) = self.elems.get(index.get().unwrap()) {
            data.is_some()
        } else {
            false
        }
    }
    #[inline]
    pub fn first_index(&self) -> Index {
        self.used.head
    }
    #[inline]
    pub fn last_index(&self) -> Index {
        self.used.tail
    }
    #[inline]
    pub fn next_index(&self, index: Index) -> Index {
        if index.is_none() { return index; }
        if let Some(node) = self.nodes.get(index.get().unwrap()) {
            node.next
        } else {
            Index::new()
        }
    }
    #[inline]
    pub fn prev_index(&self, index: Index) -> Index {
        if index.is_none() { return index; }
        if let Some(node) = self.nodes.get(index.get().unwrap()) {
            node.prev
        } else {
            Index::new()
        }
    }
    #[inline]
    pub fn get_first(&self) -> Option<&T> {
        self.get(self.first_index())
    }
    #[inline]
    pub fn get_last(&self) -> Option<&T> {
        self.get(self.last_index())
    }
    #[inline]
    pub fn get(&self, index: Index) -> Option<&T> {
        if index.is_none() { return None; }
        self.elems.get(index.get().unwrap())?.as_ref()
    }
    #[inline]
    pub fn get_mut_first(&mut self) -> Option<&mut T> {
        self.get_mut(self.first_index())
    }
    #[inline]
    pub fn get_mut_last(&mut self) -> Option<&mut T> {
        self.get_mut(self.last_index())
    }
    #[inline]
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        if let Some(ndx) = index.get() {
            if ndx < self.len() {
                self.elems[ndx].as_mut()
            } else {
                None
            }
        } else {
            None
        }
    }
    #[inline]
    // Peek at next element data, if any
    pub fn peek_next(&self, index: Index) -> Option<&T> {
        self.get(self.next_index(index))
    }
    #[inline]
    // Peek at previous element data, if any
    pub fn peek_prev(&self, index: Index) -> Option<&T> {
        self.get(self.prev_index(index))
    }
    #[inline]
    pub fn contains(&self, elem: T) -> bool
    where T: PartialEq {
        self.elems.contains(&Some(elem))
    }
    pub fn insert_first(&mut self, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_first(pos, Some(true));
        Index::from(pos)
    }
    pub fn insert_last(&mut self, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_last(pos, Some(true));
        Index::from(pos)
    }
    pub fn insert_before(&mut self, index: Index, elem: T) -> Index {
        if index.is_none() {
            return self.insert_first(elem);
        }
        let ndx = index.get().unwrap();
        let pos = self.new_node(Some(elem));
        self.linkin_this_before_that(pos, ndx);
        Index::from(pos)
    }
    pub fn insert_after(&mut self, index: Index, elem: T) -> Index {
        if index.is_none() {
            return self.insert_last(elem);
        }
        let ndx = index.get().unwrap();
        let pos = self.new_node(Some(elem));
        self.linkin_this_after_that(pos, ndx);
        Index::from(pos)
    }
    pub fn remove_first(&mut self) -> Option<T> {
        self.remove(self.first_index())
    }
    pub fn remove_last(&mut self) -> Option<T> {
        self.remove(self.last_index())
    }
    pub fn remove(&mut self, index: Index) -> Option<T> {
        if index.is_none() { return None; }
        let ndx = index.get().unwrap();
        if ndx >= self.nodes.len() { return None; }
        let elem = self.remove_elem_at_index(ndx);
        debug_assert!(self.elems[ndx].is_none());
        self.linkout(ndx, Some(true));
        self.insert_free_node(ndx);
        elem
    }
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter { list: &self, curr: self.first_index() }
    }
    pub fn to_vec(&self) -> Vec<&T> {
        self.iter().filter_map(Option::Some).collect()
    }
    pub fn from(vec: &mut Vec<T>) -> IndexList<T> {
        let mut list = IndexList::<T>::new();
        vec.drain(..).for_each(|elem| {
            list.insert_last(elem);
        });
        list
    }
    /// Remove some unused indexes at the end by truncating, if any
    pub fn trim_safe(&mut self) {
        let removed: Vec<usize> = (self.len()..self.capacity())
            .rev()
            .take_while(|&i| self.is_free(i))
            .collect();
        removed.iter().for_each(|&i| {
            self.linkout(i, Some(false));
        });
        if removed.len() > 0 {
            let left = self.capacity() - removed.len();
            self.nodes.truncate(left);
            self.elems.truncate(left);
        }
    }
    /// Remove all unused elements by moving indexes then truncating
    /// Note that this call may invalidate some indexes
    pub fn trim_swap(&mut self) {
        let need = self.size;
        // destination is all free node indexes below the needed limit
        let dst: Vec<usize> = self.elems[..need]
            .iter()
            .enumerate()
            .filter(|(n, e)| e.is_none() && n < &need)
            .map(|(n, _e)| n)
            .collect();
        // source is all used node indexes above the needed limit
        let src: Vec<usize> = self.elems[need..]
            .iter()
            .enumerate()
            .filter(|(_n, e)| e.is_some())
            .map(|(n, _e)| n + need)
            .collect();
        debug_assert_eq!(dst.len(), src.len());
        src.iter()
            .zip(dst.iter())
            .for_each(|(s, d)| self.replace_dest_index(*s, *d));
        self.free.new_both(Index::new());
        self.elems.truncate(need);
        self.nodes.truncate(need);
    }
    /// Add the elements of the other list after the elements of this list
    pub fn append(&mut self, other: &mut IndexList<T>) {
        while let Some(elem) = other.remove_first() {
            self.insert_last(elem);
        }
    }
    /// Add the element of the other list before the element of this list
    pub fn prepend(&mut self, other: &mut IndexList<T>) {
        while let Some(elem) = other.remove_last() {
            self.insert_first(elem);
        }
    }
    /// Split the elements at this index and after to a new list
    pub fn split(&mut self, index: Index) -> IndexList<T> {
        let mut list = IndexList::<T>::new();
        if index.is_none() {
            return list;
        }
        loop {
            let last = self.last_index();
            if last.is_none() {
                break;
            }
            list.insert_first(self.remove_last().unwrap());
            if last == index {
                break;
            }
        }
        list
    }

    #[inline]
    fn is_used(&self, at: usize) -> bool {
        self.elems[at].is_some()
    }
    fn is_free(&self, at: usize) -> bool {
        self.elems[at].is_none()
    }
    #[inline]
    fn get_mut_indexnode(&mut self, at: usize) -> &mut IndexNode {
        &mut self.nodes[at]
    }
    #[inline]
    fn get_indexnode(&self, at: usize) -> &IndexNode {
        &self.nodes[at]
    }
    #[allow(dead_code)]
    #[inline]
    fn set_prev(&mut self, index: Index, new_prev: Index) -> Index {
        if let Some(at) = index.get() {
            self.get_mut_indexnode(at).new_prev(new_prev)
        } else {
            index
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn set_next(&mut self, index: Index, new_next: Index) -> Index {
        if let Some(at) = index.get() {
            self.get_mut_indexnode(at).new_next(new_next)
        } else {
            index
        }
    }
    #[inline]
    fn insert_elem_at_index(&mut self, at: usize, elem: Option<T>) {
        self.elems[at] = elem;
        self.size += 1;
    }
    #[inline]
    fn remove_elem_at_index(&mut self, at: usize) -> Option<T> {
        self.size -= 1;
        self.elems[at].take()
    }
    fn new_node(&mut self, elem: Option<T>) -> usize {
        if let Some(pos) = self.remove_free_node() {
            self.insert_elem_at_index(pos, elem);
            pos
        } else {
            let pos = self.nodes.len();
            self.nodes.push(IndexNode::new());
            self.elems.push(elem);
            self.size += 1;
            pos
        }
    }
    fn remove_free_node(&mut self) -> Option<usize> {
        if let Some(head_pos) = self.free.head.get() {
            self.linkout(head_pos, Some(false));
            Some(head_pos)
        } else {
            None
        }
    }
    fn insert_free_node(&mut self, at: usize) {
        debug_assert!(self.is_free(at));
        self.linkin_last(at, Some(false));
    }
    fn linkin_first(&mut self, this: usize, is_used: Option<bool>) {
        let is_used = is_used.unwrap_or(self.is_used(this));
        let old_head_ndx = if is_used { self.used.head } else { self.free.head };
        if let Some(old_head_pos) = old_head_ndx.get() {
            let old_head = self.get_mut_indexnode(old_head_pos);
            let old_head_prev = old_head.new_prev(Index::from(this));
            debug_assert_eq!(old_head_prev.get(), None);
            let this_node = self.get_mut_indexnode(this);
            let old_next = this_node.new_next(Index::from(old_head_pos));
            debug_assert_eq!(old_next.get(), None);
        }
        let list = if is_used { &mut self.used } else { &mut self.free };
        if list.new_head(Index::from(this)).is_none() {
            if list.is_empty() {
                list.new_both(Index::from(this));
            } else {
                let old_prev_ndx = list.new_tail(Index::from(this));
                debug_assert_eq!(old_prev_ndx.get(), None);
            }
        }
    }
    fn linkin_last(&mut self, this: usize, is_used: Option<bool>) {
        let is_used = is_used.unwrap_or(self.is_used(this));
        let old_tail_ndx = if is_used { self.used.tail } else { self.free.tail };
        if let Some(old_tail_pos) = old_tail_ndx.get() {
            let old_tail = self.get_mut_indexnode(old_tail_pos);
            let old_tail_next = old_tail.new_next(Index::from(this));
            debug_assert_eq!(old_tail_next.get(), None);
            let this_node = self.get_mut_indexnode(this);
            let old_prev = this_node.new_prev(Index::from(old_tail_pos));
            debug_assert_eq!(old_prev.get(), None);
        }
        let list = if is_used { &mut self.used } else { &mut self.free };
        if list.new_tail(Index::from(this)).is_none() {
            if list.is_empty() {
                list.new_both(Index::from(this));
            } else {
                let old_head = list.new_head(Index::from(this));
                debug_assert_eq!(old_head.get(), None);
            }
        }
    }
    fn linkin_this_before_that(&mut self, this: usize, that: usize) {
        let next = self.get_mut_indexnode(that);
        let before = next.new_prev(Index::from(this));
        let node = self.get_mut_indexnode(this);
        node.new_prev(before);
        node.new_next(Index::from(that));
        if let Some(prev_pos) = before.get() {
            let prev = self.get_mut_indexnode(prev_pos);
            let old_next_ndx = prev.new_next(Index::from(this));
            debug_assert_eq!(old_next_ndx.get(), Some(that));
        } else {
            let old_next_ndx = self.used.new_head(Index::from(this));
            debug_assert_eq!(old_next_ndx, Index::from(that));
        }
    }
    fn linkin_this_after_that(&mut self, this: usize, that: usize) {
        let prev = self.get_mut_indexnode(that);
        let next_opt = prev.new_next(Index::from(this));
        let node = self.get_mut_indexnode(this);
        node.new_next(next_opt);
        node.new_prev(Index::from(that));
        if let Some(next_pos) = next_opt.get() {
            let next = self.get_mut_indexnode(next_pos);
            let old_prev = next.new_prev(Index::from(this));
            debug_assert_eq!(old_prev, Index::from(that));
        } else {
            let old_prev = self.used.new_tail(Index::from(this));
            debug_assert_eq!(old_prev, Index::from(that));
        }
    }
    fn linkout(&mut self, this: usize, is_used: Option<bool>) {
        let node = self.get_mut_indexnode(this);
        let next_opt = node.new_next(Index::new());
        let prev_opt = node.new_prev(Index::new());
        if let Some(next_pos) = next_opt.get() {
            let next = self.get_mut_indexnode(next_pos);
            let old_prev = next.new_prev(prev_opt);
            debug_assert_eq!(old_prev, Index::from(this));
        }
        if let Some(prev_pos) = prev_opt.get() {
            let prev = self.get_mut_indexnode(prev_pos);
            let old_next = prev.new_next(next_opt);
            debug_assert_eq!(old_next, Index::from(this));
        }
        let is_used = is_used.unwrap_or(self.is_used(this));
        let list = if is_used { &mut self.used } else { &mut self.free };
        if next_opt.is_none() {
            let old_prev = list.new_tail(prev_opt);
            debug_assert_eq!(old_prev, Index::from(this));
        }
        if prev_opt.is_none() {
            let old_next = list.new_head(next_opt);
            debug_assert_eq!(old_next, Index::from(this));
        }
    }
    fn replace_dest_index(&mut self, src: usize, dst: usize) {
        self.linkout(dst, Some(false));
        let src_node = self.get_indexnode(src);
        let src_next_opt = src_node.next;
        let src_prev_opt = src_node.prev;
        self.linkout(src, Some(true));
        self.elems[dst] = self.elems[src].take();
        if let Some(next_pos) = src_next_opt.get() {
            self.linkin_this_before_that(dst, next_pos);
        } else if let Some(prev_pos) = src_prev_opt.get() {
            self.linkin_this_after_that(dst, prev_pos);
        } else {
            self.linkin_first(dst, Some(true));
        }
    }
}

impl<T> fmt::Display for IndexList<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elems: Vec<String> = self.iter().map(|x| format!("{:?}", x)).collect();
        write!(f, "[ {} ]", elems.join(" >< "))
    }
}

impl<T> From<T> for IndexList<T> {
    fn from(elem: T) -> IndexList<T> {
        let mut list = IndexList::new();
        list.insert_last(elem);
        list
    }
}

pub struct Iter<'a, T> {
    list: &'a IndexList<T>,
    curr: Index,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.curr);
        self.curr = self.list.next_index(self.curr);
        item
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let my_len = self.list.len();
        (my_len, Some(my_len))
    }
}
