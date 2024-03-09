/// ITERATORS
/// The for-loop over a range was using an iterator (0..n is actually similar to the Python 3 range function).
/// An iterator is easy to define informally.
/// It is an 'object' with a next method which returns an Option.
/// As long as that value is not None, we keep calling next:

// iter1.rs
/// This may seem an inefficient way to define a for-loop, but rustc does crazy-ass optimizations in release mode and it will be just as fast as a while loop.
fn iter1() {
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    // assert_eq!(iter.next(), Some(3)); // => panic
    assert_eq!(iter.next(), None);
}

// iter2.rs
// iterate over an array
fn iter2() {
    let arr = [10, 20, 30];
    for i in arr { // gets converted to an iterator automatically
        println!("{}", i);
    }
}
// iter3.rs
// it's more efficient to iterate over an array than doing for i in 0..slice.len() {}
fn iter3() {
    let arr = [10, 20, 30];
    for i in arr.iter() { // works without .iter()
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}

pub fn main() {
    iter1();
    iter2();
    iter3();
}