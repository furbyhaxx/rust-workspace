/// We'll return to slice methods again, but first: vectors.
/// These are re-sizeable arrays and behave much like Python List and C++ std::vector.
/// The Rust type Vec (pronounced 'vector') behaves very much like an slice in fact;
/// the difference is that you can append extra values to a vector - note that it must be declared as mutable.

fn create_vector() {
    let mut vec = Vec::new();

    let mut vec2 = Vec::with_capacity(10); // can hold up to 10 items without relocating

    // or with the macro and initial data
    let vec3 = vec![1,2,3,4];
}

fn basic_vector_functions() {
    let mut vec = Vec::new();


    // add to end
    vec.push(10);
    vec.push(20);
    vec.push(30);

    // get element
    let first = vec[0];  // will panic if out-of-range
    let maybe_first = vec.get(0);

    // remove from index
    vec.remove(0);

    // remove from bottom/end
    vec.pop();

    // add / combine two vectors
    vec.extend(0..2);
    vec.extend(vec![1,2,3]);
}

fn advanced_vector_functions() {
    let mut vec = vec![1, 10, 5, 1, 2, 11, 2, 40];

    // sort a vector
    vec.sort(); // inplace
    vec.sort_by(|k| k.abs());

    // remove duplicates
    vec.dedup();
    vec.dedup_by(|a, b| a < b);
}

// vec1.rs
fn vec1() {
    let mut v = Vec::new(); // mut is needed
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}

// vec2.rs
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn vec2() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    // borrow operator & is coercing the vector into a slice.
    // And it makes complete sense, because the vector is also looking after an array of values, with the difference that the array is allocated dynamically.
    let slice = &v[1..];
    println!("slice is {:?}", slice);
}

// about stack and heap
// If you come from a dynamic language, now is time for that little talk.
// In systems languages, program memory comes in two kinds: the stack and the heap.
// It is very fast to allocate data on the stack, but the stack is limited; typically of the order of megabytes.
// The heap can be gigabytes, but allocating is relatively expensive, and such memory must be freed later.
// In so-called 'managed' languages (like Java, Go and the so-called 'scripting' languages) these details are hidden from you by
// that convenient municipal utility called the garbage collector.
// Once the system is sure that data is no longer referenced by other data, it goes back into the pool of available memory.
//
// Generally, this is a price worth paying.
// Playing with the stack is terribly unsafe, because if you make one mistake you can override the return address of the current function,
// and you die an ignominious death or (worse) got pwned by some guy living in his Mom's basement in Minsk.

pub fn main() {
    vec1();
    vec2();
}