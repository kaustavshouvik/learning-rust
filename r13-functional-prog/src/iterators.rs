fn iterator_mutability() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    // Calling 'next()' modifies the internal state
    // that's why we mark the iterator as mutable.
    let first = v_iter.next();
    assert_eq!(first, Some(&1));
}

// Takes ownership of an iterator,
// where each item is a owned 'String'.
fn take_iter(iter: impl Iterator<Item = String>) {
    for s in iter {
        println!("{s}");
    }
}

pub fn iterators() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    // Converting a iterator:
    let v_iter = v_iter.map(|x| x + 1);

    // 'for' takes ownership of the iterator.
    // for x in v_iter {
    //     println!("{x}");
    // }

    // Adaptors:
    //  -> Consuming adaptors: Which uses up an iterator.
    //  -> Iterator adaptors: Takes in an iterator
    //      and changes some aspect of the original iterator.

    // 'map' is a iterator adaptor.
    // Nothing happens until we call a consuming adaptor on it.
    // By "nothing", it means:
    //  -> The closure is not called.

    // 'collect' is a consuming adaptor.
    let c: Vec<_> = v_iter.collect();
    println!("{c:?}");

    iterator_mutability();

    take_iter(std::env::args());
}
