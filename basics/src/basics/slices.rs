
fn create_slice() {
    // define an array
    let arr = [10, 20, 30];
    let slice = &arr;

    // oneliner
    let slice2 = &[10, 20, 30];
}

// slice1.rs
fn print_slice() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

fn optional_values() {
    /// check if an index or value exists
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());
}

fn optional_values_unwrap() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        // Note the * - the precise type inside the Some is &i32, which is a reference.
        // We need to dereference this to get back to a i32 value.
        *maybe_last.unwrap()
    } else {
        -1
    };
    // shortcut
    let last = *slice.get(5).unwrap_or(&-1);
}

/// SLICE - window
fn slice_window() {
    let arr = [1,2,3,4,5,6,7,8,9];
    let slice = &arr;

    for s in slice.windows(2) {
        println!("widnow: [{:?}]", s);
    }
}

fn slice_chunks() {
    let arr = [1,2,3,4,5,6,7,8,9];
    let slice = &arr;

    for s in slice.chunks(2) {
        println!("chunk: [{:?}]", s);
    }
}

pub fn main() {
    print_array();
    print_slice();
    optional_values();
    optional_values_unwrap();
}