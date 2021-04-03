use std::fmt;

pub type Index = usize;

#[derive(Clone, Debug, Default)]
struct IndexNode {
    next: Option<Index>,
    prev: Option<Index>,
}

impl IndexNode {
    #[inline]
    pub fn new() -> IndexNode {
        IndexNode { next: None, prev: None }
    }
    #[inline]
    pub fn new_next(&mut self, next: Option<Index>) -> Option<Index> {
        let old_next = self.next;
        self.next = next;
        old_next
    }
    #[inline]
    pub fn new_prev(&mut self, prev: Option<Index>) -> Option<Index> {
        let old_prev = self.prev;
        self.prev = prev;
        old_prev
    }
}

impl fmt::Display for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}<->{:?}", self.next, self.prev)
    }
}

#[derive(Clone, Debug, Default)]
struct IndexEnds {
    head: Option<Index>,
    tail: Option<Index>,
}

impl IndexEnds {
    #[inline]
    pub fn new() -> Self {
        IndexEnds { head: None, tail: None }
    }
    #[inline]
    pub fn clear(&mut self) {
        self.new_both(None);
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    #[inline]
    pub fn new_head(&mut self, head: Option<Index>) -> Option<Index> {
        let old_head = self.head;
        self.head = head;
        old_head
    }
    #[inline]
    pub fn new_tail(&mut self, tail: Option<Index>) -> Option<Index> {
        let old_tail = self.tail;
        self.tail = tail;
        old_tail
    }
    #[inline]
    pub fn new_both(&mut self, both: Option<Index>) {
        self.head = both;
        self.tail = both;
    }
}

impl fmt::Display for IndexEnds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}=> ... <={:?}", self.head, self.tail)
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
        if let Some(data) = self.elems.get(index) {
            data.is_some()
        } else {
            false
        }
    }
    #[inline]
    pub fn first_index(&self) -> Option<Index> {
        self.used.head
    }
    #[inline]
    pub fn last_index(&self) -> Option<Index> {
        self.used.tail
    }
    #[inline]
    pub fn next_index(&self, index: Index) -> Option<Index> {
        self.nodes.get(index)?.next
    }
    #[inline]
    pub fn prev_index(&self, index: Index) -> Option<Index> {
        self.nodes.get(index)?.prev
    }
    #[inline]
    pub fn get_first(&self) -> Option<&T> {
        self.get_index(self.first_index()?)
    }
    #[inline]
    pub fn get_last(&self) -> Option<&T> {
        self.get_index(self.last_index()?)
    }
    #[inline]
    pub fn get_index(&self, index: Index) -> Option<&T> {
        self.elems.get(index)?.as_ref()
    }
    #[inline]
    pub fn get_mut_first(&mut self) -> Option<&mut T> {
        self.get_mut_index(self.first_index()?)
    }
    #[inline]
    pub fn get_mut_last(&mut self) -> Option<&mut T> {
        self.get_mut_index(self.last_index()?)
    }
    #[inline]
    pub fn get_mut_index(&mut self, index: Index) -> Option<&mut T> {
        if index < self.len() {
            self.elems[index].as_mut()
        } else {
            None
        }
    }
    #[inline]
    // Peek at next element data, if any
    pub fn peek_next(&self, index: Index) -> Option<&T> {
        self.get_index(self.next_index(index)?)
    }
    #[inline]
    // Peek at previous element data, if any
    pub fn peek_prev(&self, index: Index) -> Option<&T> {
        self.get_index(self.prev_index(index)?)
    }
    #[inline]
    pub fn contains(&self, elem: T) -> bool
    where T: PartialEq {
        self.elems.contains(&Some(elem))
    }
    pub fn insert_first(&mut self, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_first(pos, Some(true));
        pos
    }
    pub fn insert_last(&mut self, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_last(pos, Some(true));
        pos
    }
    pub fn insert_before(&mut self, index: Index, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_this_before_that(pos, index);
        pos
    }
    pub fn insert_after(&mut self, index: Index, elem: T) -> Index {
        let pos = self.new_node(Some(elem));
        self.linkin_this_after_that(pos, index);
        pos
    }
    pub fn remove_first(&mut self) -> Option<T> {
        self.remove_index(self.first_index()?)
    }
    pub fn remove_last(&mut self) -> Option<T> {
        self.remove_index(self.last_index()?)
    }
    pub fn remove_index(&mut self, index: Index) -> Option<T> {
        if index >= self.nodes.len() { return None; }
        let elem = self.remove_elem_at_index(index);
        debug_assert!(self.elems[index].is_none());
        self.linkout(index, Some(true));
        self.insert_free_node(index);
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
        let removed: Vec<Index> = (self.len()..self.capacity())
            .rev()
            .take_while(|&x| self.is_index_free(x))
            .collect();
        removed.iter().for_each(|&x| {
            self.linkout(x, Some(false));
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
        let dst: Vec<Index> = self.elems[..need]
            .iter()
            .enumerate()
            .filter(|(n, e)| e.is_none() && n < &need)
            .map(|(n, _e)| n)
            .collect();
        // source is all used node indexes above the needed limit
        let src: Vec<Index> = self.elems[need..]
            .iter()
            .enumerate()
            .filter(|(_n, e)| e.is_some())
            .map(|(n, _e)| n + need)
            .collect();
        debug_assert_eq!(dst.len(), src.len());
        src.iter()
            .zip(dst.iter())
            .for_each(|(s, d)| self.replace_dest_index(*s, *d));
        self.free.new_both(None);
        self.elems.truncate(need);
        self.nodes.truncate(need);
    }
    /// Add the elements from the other list after the elements of this
    pub fn append(&mut self, other: &mut IndexList<T>) {
        if other.is_empty() { return; }
        other.trim_swap(); // Do not to copy any unused nodes.
        let offset = self.capacity() as Index;
        self.elems.append(&mut other.elems);
        self.nodes.append(&mut other.nodes);
        self.nodes[offset..].iter_mut().for_each(|node| {
            if let Some(next_pos) = node.next {
                node.next = Some(next_pos + offset);
            }
            if let Some(prev_pos) = node.prev {
                node.prev = Some(prev_pos + offset);
            }
        });
        let other_head_pos = other.first_index().unwrap() + offset;
        self.set_prev(other_head_pos, self.last_index());
        self.set_next(self.last_index().unwrap(), Some(other_head_pos));
        let other_tail_pos = other.last_index().unwrap() + offset;
        self.used.new_tail(Some(other_tail_pos));
        self.size += other.size;
    }

    #[inline]
    fn is_index_free(&self, index: Index) -> bool {
        self.elems[index].is_none()
    }
    #[inline]
    fn get_mut_indexnode(&mut self, at: Index) -> &mut IndexNode {
        &mut self.nodes[at]
    }
    #[inline]
    fn get_indexnode(&self, at: Index) -> &IndexNode {
        &self.nodes[at]
    }
    #[inline]
    fn set_prev(&mut self, at: Index, new_prev: Option<Index>) {
        self.get_mut_indexnode(at).new_prev(new_prev);
    }
    #[inline]
    fn set_next(&mut self, at: Index, new_next: Option<Index>) {
        self.get_mut_indexnode(at).new_next(new_next);
    }
    #[inline]
    fn insert_elem_at_index(&mut self, at: Index, elem: Option<T>) {
        self.elems[at] = elem;
        self.size += 1;
    }
    #[inline]
    fn remove_elem_at_index(&mut self, at: Index) -> Option<T> {
        self.size -= 1;
        self.elems[at].take()
    }
    fn new_node(&mut self, elem: Option<T>) -> Index {
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
    fn remove_free_node(&mut self) -> Option<Index> {
        if let Some(head_pos) = self.free.head {
            self.linkout(head_pos, Some(false));
            Some(head_pos)
        } else {
            None
        }
    }
    fn insert_free_node(&mut self, this: Index) {
        debug_assert!(self.is_index_free(this));
        self.linkin_last(this, Some(false));
    }
    fn linkin_first(&mut self, this: Index, is_used: Option<bool>) {
        let is_used = is_used.unwrap_or(self.is_index_used(this));
        let old_head_opt = if is_used { self.used.head } else { self.free.head };
        if let Some(old_head_pos) = old_head_opt {
            let old_head = self.get_mut_indexnode(old_head_pos);
            let old_head_prev = old_head.new_prev(Some(this));
            debug_assert_eq!(old_head_prev, None);
            let this_node = self.get_mut_indexnode(this);
            let old_next = this_node.new_next(Some(old_head_pos));
            debug_assert_eq!(old_next, None);
        }
        let list = if is_used { &mut self.used } else { &mut self.free };
        if list.new_head(Some(this)).is_none() {
            if list.is_empty() {
                list.new_both(Some(this));
            } else {
                let old_prev = list.new_tail(Some(this));
                debug_assert_eq!(old_prev, None);
            }
        }
    }
    fn linkin_last(&mut self, this: Index, is_used: Option<bool>) {
        let is_used = is_used.unwrap_or(self.is_index_used(this));
        let old_tail_opt = if is_used { self.used.tail } else { self.free.tail };
        if let Some(old_tail_pos) = old_tail_opt {
            let old_tail = self.get_mut_indexnode(old_tail_pos);
            let old_tail_next = old_tail.new_next(Some(this));
            debug_assert_eq!(old_tail_next, None);
            let this_node = self.get_mut_indexnode(this);
            let old_prev = this_node.new_prev(Some(old_tail_pos));
            debug_assert_eq!(old_prev, None);
        }
        let list = if is_used { &mut self.used } else { &mut self.free };
        if list.new_tail(Some(this)).is_none() {
            if list.is_empty() {
                list.new_both(Some(this));
            } else {
                let old_head_opt = list.new_head(Some(this));
                debug_assert_eq!(old_head_opt, None);
            }
        }
    }
    fn linkin_this_before_that(&mut self, this: Index, that: Index) {
        let next = self.get_mut_indexnode(that);
        let before = next.new_prev(Some(this));
        let node = self.get_mut_indexnode(this);
        node.new_prev(before);
        node.new_next(Some(that));
        if let Some(prev_pos) = before {
            let prev = self.get_mut_indexnode(prev_pos);
            let old_next = prev.new_next(Some(this));
            debug_assert_eq!(old_next, Some(that));
        } else {
            let old_next = self.used.new_head(Some(this));
            debug_assert_eq!(old_next, Some(that));
        }
    }
    fn linkin_this_after_that(&mut self, this: Index, that: Index) {
        let prev = self.get_mut_indexnode(that);
        let next_opt = prev.new_next(Some(this));
        let node = self.get_mut_indexnode(this);
        node.new_next(next_opt);
        node.new_prev(Some(that));
        if let Some(next_pos) = next_opt {
            let next = self.get_mut_indexnode(next_pos);
            let old_prev = next.new_prev(Some(this));
            debug_assert_eq!(old_prev, Some(that));
        } else {
            let old_prev = self.used.new_tail(Some(this));
            debug_assert_eq!(old_prev, Some(that));
        }
    }
    fn linkout(&mut self, this: Index, is_used: Option<bool>) {
        let node = self.get_mut_indexnode(this);
        let next_opt = node.new_next(None);
        let prev_opt = node.new_prev(None);
        if let Some(next_pos) = next_opt {
            let next = self.get_mut_indexnode(next_pos);
            let old_prev = next.new_prev(prev_opt);
            debug_assert_eq!(old_prev, Some(this));
        }
        if let Some(prev_pos) = prev_opt {
            let prev = self.get_mut_indexnode(prev_pos);
            let old_next = prev.new_next(next_opt);
            debug_assert_eq!(old_next, Some(this));
        }
        let is_used = is_used.unwrap_or(self.is_index_used(this));
        let list = if is_used { &mut self.used } else { &mut self.free };
        if next_opt.is_none() {
            let old_prev = list.new_tail(prev_opt);
            debug_assert_eq!(old_prev, Some(this));
        }
        if prev_opt.is_none() {
            let old_next = list.new_head(next_opt);
            debug_assert_eq!(old_next, Some(this));
        }
    }
    fn replace_dest_index(&mut self, src: Index, dst: Index) {
        self.linkout(dst, Some(false));
        let src_node = self.get_indexnode(src);
        let src_next_opt = src_node.next;
        let src_prev_opt = src_node.prev;
        self.linkout(src, Some(true));
        self.elems[dst] = self.elems[src].take();
        if let Some(next_pos) = src_next_opt {
            self.linkin_this_before_that(dst, next_pos);
        } else if let Some(prev_pos) = src_prev_opt {
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
    curr: Option<Index>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(this) = self.curr {
            self.curr = self.list.next_index(this);
            self.list.get_index(this)
        } else {
            None
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let my_len = self.list.len();
        (my_len, Some(my_len))
    }
}
