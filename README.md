# Index List

An index list is a hybrid between a vector and a linked-list, with some of the properties of each. Every element has an index and can be accessed directly through that index. That index does not change if the element is moved in the list, nor when other elements are inserted in or removed from the list.

Old indexes will be reused in FIFO fashion, before new indexes are added.

## The index element

The list elements are placed in a vector, which is why they can be accessed directly, where each element knows the index of the element before and after it, as well as the data contained at that index. This indirection makes it easy to implement the list safely in Rust, because the traditional next and previous pointers are replaced by the indexes to them respectively.

You can think of it like this:
```rust
struct IndexElem<T> {
	next: Option<index>,
	prev: Option<index>,
	data: Option<T>,
}
```
Where an element without data is free and if either `next` or `prev` is `None` that is the end of the list in that direction.

## The element vector

Besides providing direct access to the element, the vector for the elements provide better locality between them, which is useful when walking through the list.

## Walking the list

The user walks the list be providing the current index, then getting either the next or the previous index. Therefore, unless using an iterator, accessing the data is a secondary call as the `next`, or `prev`, methods will only give you the new index.

Note that any calls to the `trim_swap` method, may invalidate the index. You can test if it is larger than the capacity. To prevent this, you can hold a reference to the list, where you store the index. However this will prevent any modification to the list while the reference is held.

## The list capacity

The index list will grow automatically as new elements are added. Old indexes will be reused before new ones get added. However the element vector does not automatically shrink. Instead it is up to the user to select opportunities for trimming the list capacity down to what is actually needed at that point in time.

There is a safe method (`trim_safe`), which may not actually shrink the list at all, because it will only free any unused indexes if they appear at the very end of the vector.

Then there is the unsafe method (`trim_swap`) which will swap the elements to move the free ones to the end of the vector and then truncate the vector. It is called unsafe because all indexes above the cut-off point of the number needed to contain all used elements will be invalidated. Therefore if the user has stored these indexes anywhere they will not return the correct data anymore.

## Unsafe

The index list has no unsafe code blocks. The reason is that it does not use pointers between the element, but their index in the vector instead. The only method that is considered unsafe is the `trim_swap` because it may change the index of the elements.

## Performance

In my simple tests the index list appears to offer roughly twice the performance of LinkedList, plus a lot of functionality that is only experimental, such as the cursor methods.
