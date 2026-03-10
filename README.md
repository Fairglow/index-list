# Index List

[![Rust](https://github.com/Fairglow/index-list/actions/workflows/rust.yml/badge.svg)](https://github.com/Fairglow/index-list/actions/workflows/rust.yml)

An index list is a hybrid between a vector and a linked-list, with some of the properties of each. Every element has an index in the vector and can be accessed directly there. This index is persistent as long as the element remains in the list and is not affected by other elements of the list. An index does not change if the element is moved in the list, nor when other elements are inserted or removed from the list.

The user is not meant to know the exact value of the index and should not create any Indexes themselves, but can safely copy an existing one. The index should only be used for the purpose of accessing the element data at that location or to traverse the list. This is why almost all methods on the Indexes are private.

When a new element is inserted in the list, its index will be returned from that method, but the user can safely ignore that value because the element is available by walking the list anyway.

Old indexes will be reused in FIFO fashion, before new indexes are added.

## The index list design

The list elements are placed in a vector, which is why they can be accessed directly, where each element knows the index of the element before and after it, as well as the data contained at that index. This indirection makes it easy to implement the list safely in Rust, because the traditional next and previous pointers are replaced by their respective indexes, which are just numbers.

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

Besides providing direct access to the element, the vector for the elements provide better memory efficiency and locality between them, which is useful when walking through the list as it is likely to mean fewer cache misses. The elements will however appear scrambled in the vector and only by walking the list can the correct order be established.

## Walking the list

To walk the list the user needs a starting index. One can be obtained from either `first_index` or `last_index` method calls. Then use either the `next_index`, or `prev_index` methods to move in the respective direction. An index is persistent in the list as long as the element is not removed and can be stored and used later. The indexes are typically not sequential as the list is traversed.

See the included [example code](examples/indexlist.rs) for how this works.

Note that any calls to the `trim_swap` method, may invalidate one or more index. It can be verified because any index greater than the `capacity` has been moved. To prevent this invalidation, you can hold a reference to the list as well as the index, but this will also block any and all modifications to the list while the reference is held.

## The list capacity

The index list will grow automatically as new elements are added. Old indexes will be reused before new ones get added. However the element vector does not automatically shrink. Instead it is up to the user to select opportunities for trimming the list capacity down to what is actually needed at that point in time.

There is a safe method (`trim_safe`), which may not actually shrink the list at all, because it will only free any unused indexes if they appear at the very end of the vector.

Then there is the unsafe method (`trim_swap`) which will swap the elements to move the free ones to the end of the vector and then truncate the vector. It is called unsafe because all indexes above the cut-off point of the number needed to contain all used elements will be invalidated. Therefore if the user has stored these indexes anywhere they will not return the correct data anymore.

## Mutable iterator

A mutable iterator is available via the `iter_mut` feature flag. To enable it, add the following to your `Cargo.toml`:

```toml
[dependencies]
index_list = { version = "0.3.0", features = ["iter_mut"] }
```

This feature is guarded behind a feature flag because the implementation of `iter_mut` uses a small block of `unsafe` code to satisfy the borrow checker. While it has been carefully written and tested, it is important for users to be aware of this.

If you prefer to avoid `unsafe` code altogether, you can still safely mutate elements in the list with the following pattern:

```rust
let mut index = list.first_index();
while index.is_some() {
    if let Some(elem) = list.get_mut(index) {
        // mutate elem
    }
    index = list.next_index(index);
}
```

A full example of this pattern is available in the [iter_mut alternative](examples/iter_mut_alternative.rs) example source code.

## Safe Rust

The core of this crate is implemented in 100% safe Rust, with no `unsafe` code blocks. The reason is that it does not use pointers between the elements, but their index in the vector instead.

However, there are two things to be aware of:

1. The `trim_swap` method, while being 100% safe, is considered "unsafe" from a logical point of view because it can invalidate indexes. If you have cached an index somewhere, it may point to a different element after `trim_swap` is called. Use this method with care.
2. The optional `iter_mut` feature uses a small `unsafe` block to create a mutable iterator. This is a common pattern for this kind of data structure, but it is important to be aware of it.

## Performance

This crate comes with a comprehensive benchmark suite that compares `IndexList` against several standard library collections and other similar crates. For details on how to run the benchmarks and interpret the results, see the [Benchmarking](#benchmarking) section below.

## Benchmarking

`IndexList` includes a set of benchmarks powered by `criterion`. These benchmarks are designed to evaluate the performance of various operations and compare it against other list-like data structures.

### Available Benchmarks

- **head:** Measures the performance of adding and removing elements from the front of the list.
- **tail:** Measures the performance of adding and removing elements from the back of the list.
- **body:** Measures the performance of inserting and removing elements in the middle of the list.
- **random:** Measures the performance of inserting and removing elements at random positions.
- **walk:** Measures the performance of iterating through the list using `next_index` and `prev_index`.
- **iter:** Measures the performance of iterating through the list using the `iter()` method.
- **iter_mut:** Measures the performance of mutable iteration using the `iter_mut()` method.

> NOTE: These benchmarks may not reflect any real-life performance difference and you are urged to evaluate this in your own use-case rather than relying on the figures provided by the included benchmarks.

### How to Run Benchmarks

To run the standard benchmarks, use the following command:

```shell
cargo bench
```

To include the `iter_mut` benchmarks, you need to enable the `iter_mut` feature:

```shell
cargo bench --features="iter_mut"
```

### Benchmark Comparison Table

After running the benchmarks, you can generate a summary table to easily compare the performance of the different implementations. To do this, run the `bench-table` example:

```shell
cargo run --example bench-table
```

This will parse the benchmark results and print a compact, formatted table to the console.

You can also compare the results from the `base` and `new` benchmark runs:

```shell
cargo run --example bench-table -- --base --new
```

## License

This project is licensed under the [Mozilla Public License, v. 2.0](LICENSE).

## Reasons to use IndexList

- Data that is frequently inserted or removed from the body of the list (not the ends).
- Use-case where walking the list is required.
- Data that is reordered often, or sorted.
- Need persistent indexes even when data is inserted or removed.
- Want to maintain skip elements for taking larger steps through the list.
- Need to cache certain elements for fast retrieval, without holding a reference to it.

## Reasons to use other alternatives

- Data that is mainly inserted and removed at the ends of the list, then VecDeque is likely a better alternative.
- Merges and splits of the lists are common; these are heavy `O(n)` operations in the IndexList design. The LinkedList is likely much better in this respect.
- When handling lists longer than 4 billion entries, as this list is limited to 32-bit indexes.
- When you need to shrink the list often, because `trim_swap` is expensive and has the side-effect of potentially invalidating indexes. For instance a LinkedList does not require trimming at all.

This is not an exhaustive list of alternatives, and I may have missed important choices, but these were the ones that I was aware of at the time of writing this.

- [`std::collections::LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [`std::collections::VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`pie_core::PieList`](https://docs.rs/pie_core/0.2.3/pie_core/struct.PieList.html)

### Reasons to use PieList over IndexList

PieList[^1] shares most of the reasons for picking IndexList, but with these notable improvements:

- Multiple lists of the same kind.
- O(1) split and merge for lists.
- Small data size, where data is typically always accessed when traversing the list.

The PieList is optimized for the foundation of a fibonacci heap, with a priority queue use-case.

[^1]: PieList is a part of the [pie_core](https://github.com/Fairglow/pie_core) crate.
