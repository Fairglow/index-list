# Index List

An index list is a hybrid between a vector and a linked-list, with some of the properties of each. Every element has an index in the vector and can be accessed directly there. This index is persistent as long as the element remains in the list and is not affected by other elements of the list. An index does not change if the element is moved in the list, nor when other elements are inserted or removed from the list.

The user is not meant to know the exact value stored inside of the index and should not create any Indexes, but can safely copy an existing one. The index should only be used for the purpose of accessing the element data at that location or to traverse the list. This is why almost all methods on the Indexes are private.

When a new element is inserted in the list, its index will be returned from that method, but the user can safely ignore that value as the element can be found anyway by walking the list.

Old indexes will be reused in FIFO fashion, before new indexes are added.

## The index list design

The list elements are placed in a vector, which is why they can be accessed directly, where each element knows the index of the element before and after it, as well as the data contained at that index. This indirection makes it easy to implement the list safely in Rust, because the traditional next and previous pointers are replaced by their respective indexes.

You can think of a list node like this:
```rust
struct IndexElem<T> {
	next: Option<u32>,
	prev: Option<u32>,
	data: Option<T>,
}
```
Where an element without data is free and if either `next` or `prev` is `None` then that is the end of the list in that direction.

## The element vector

Besides providing direct access to the element, the vector for the elements provide better locality between them, which is useful when walking through the list as it is likely to mean fewer cache misses. The element will however appear scrambled in the vector and only by walking the list can the correct order be established.

## Walking the list

To walk the list the user needs a starting index. One can be obtained from either `first_index` or `last_index` method calls. Then use either the the `next_index`, or `prev_index` methods to move in the respective direction. An index is persistent in the list as long as the element is not removed and can be stored and used later. The indexes are typically not sequential as the list is traversed.

Note that any calls to the `trim_swap` method, may invalidate one or more index. It van be verified because any index greater than the `capacity` has been moved. To prevent this invalidation, you can hold a reference to the list as well as the index, but this will also block any and all modifications to the list while the reference is held.

## The list capacity

The index list will grow automatically as new elements are added. Old indexes will be reused before new ones get added. However the element vector does not automatically shrink. Instead it is up to the user to select opportunities for trimming the list capacity down to what is actually needed at that point in time.

There is a safe method (`trim_safe`), which may not actually shrink the list at all, because it will only free any unused indexes if they appear at the very end of the vector.

Then there is the unsafe method (`trim_swap`) which will swap the elements to move the free ones to the end of the vector and then truncate the vector. It is called unsafe because all indexes above the cut-off point of the number needed to contain all used elements will be invalidated. Therefore if the user has stored these indexes anywhere they will not return the correct data anymore.

## Unsafe

The index list has no unsafe code blocks. The reason is that it does not use pointers between the element, but their index in the vector instead.

However the `trim_swap` method is considered unsafe, but for a different reason and that is because it may change the index of some elements. Therefore any stored indexes may be invalid after the method call, or point to a different element. Use it wisely and make sure no such indexes are kept at that time.

## Performance

In my simple benchmark tests the index list appears to offer roughly twice the performance of LinkedList, plus a some functionality that is only experimental, such as the cursor methods.

This may not reflect any real-life performance difference and you are urged to evaluate this in your own use-case rather than relying on the figures provided by the included benchmarks.

## Reasons to use IndexList

* Data is frequently inserted or removed from the body of the list (not the ends).
* Data is reordered often, or sorted.
* Need persistent indexes even when data is inserted or removed.
* Want to maintain skip elements for taking larger steps through the list.
* Need to cache certain elements for fast retrieval, without holding a reference to it.

## Reasons to use an alternative

* Data is mainly inserted and removed at the ends of the list, then VecDeque is likely a better alternative.
* Merges and splits of the lists are common; these are heavy `O(n)` operations in this design.
* When handling lists longer than 4 billion entries, as this list is limited to 32-bit indexes.
* When you need to shrink the list often, because `trim_swap` is expensive and has the side-effect of potentially invalidating indexes.

This is not an exhaustive list of alternatives, and I may have missed important choices, but these were the ones that I was aware of at the time of writing this.

* `std::collection::LinkedList`
* `std::collections::vec_deque::VecDeque`
* `Vec`
