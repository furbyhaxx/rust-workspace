// arrays and slices

/// All statically-typed languages have arrays, which are values packed nose to tail in memory. Arrays are indexed from zero:
fn array1() {
    // accessing arr[5] throws a compile error
    // arrays are fixed size
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
}

// array2.rs
// read as: slice of i32
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() { // <- the magic
        res += values[i]
    }
    res
}

fn array2() {
    let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}

pub fn main() {
    array1();
    array2();
}