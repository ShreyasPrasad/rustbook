#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    /*
        We need to make v1_iter mutable, since calling next
        mutates internal state for the iterator. A for loop
        takes ownership behind the scenes, so we don't need 
        mut there.

        The iter method produces an iterator over immutable
        references to the vector values. If we want to create
        an iterator that returns owned values, we can call
        into_iter. If we want to iterate over mutable references,
        call iter_mut.

        Each call to next eats up an item from the iterator.
    */
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
