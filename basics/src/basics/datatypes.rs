use std::f64::INFINITY;

fn primitives() {
    let i1: i32 = 123;
}

pub fn main() {
    // https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types

    /// booleans
    let bt = true;
    let bf = false;

    /// chars
    /// single quotes
    let ca = 'a';
    let cb = 'b';
    let ckeyboard = '⌨';

    /// integers
    /// These types include i8, i16, i32, i64, isize, u8, u16, u32, u64, usize.
    /// The letter denotes whether it is signed (i) or unsigned (u), and the number denotes the size of the integer.
    /// So the type i8 is an 8 bit, integer and a u64 is an unsigned, 64 bit integer.
    /// isize and usize are dependent upon the architecture of the computer.
    /// unsigned integers start from zero
    /// signed start from below zero
    /// i8 = -128/+127
    /// u8 = 0/+255
    /// i16 = -32768/+32767
    /// u16 = 0/+65535
    /// i32 = -2147483648/+2147483647
    /// u32 = 0/+4294967295
    /// i64 = -9223372036854775808/+9223372036854775807
    /// u64 = 0/+18446744073709551615
    /// usize = The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    /// isize = The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    let ix: i8 = -5;
    let life: u32 = 4294967295;
    let jenny = 8675309;

    /// ranges
    let rexc = 0..3; // exclusive range 0,1,2
    let rinc = 0..=3; // inclusive range 0,1,2,3

    /// floats
    let infinity = f64::INFINITY;
    let neginfinity = f64::NEG_INFINITY;
    let fnan = f64::NAN;
    let fmall: f32 = 1.25;
    let fbig: f64 = 1.25;

    /// Strings
    /// A [str] is a "string slice", and is the most primitive string type.
    let str = "Hello! I'm a &str";
    let string = String::from("Hello! I'm a String");
    println!("The value of our variable is: {}", str);

    /// arrays
    /// let name: [type; size] = [elem1, elem2, elem3, elem4];
    let array: [i32; 5] = [0, 1, 2, 3, 4];
    let array2 = ["str", "str2", "str3"];
    let array2 = ['a', 'b', 'c'];

    println!("The first element of the array is: {}", array[0]);

    let mut counter = 0;
    for x in array.iter(){
        println!("The element at index {} is {}", counter, x);
        counter += 1;
    }

    /// slices
    let array3: [i32; 5] = [0, 1, 2, 3, 4];

    let slice = &array3[0..3]; // This will select the elements at index 0, 1, and 2. The first number is inclusive and second number is exclusive.

    for x in slice {
        println!("x is {}", x);
    }

    /// tuples
    /// Tuples are finite, heterogeneous, sequences.
    /// They have a size, a fixed number of elements.
    /// They are heterogeneous
    /// They can contain multiple different types
    let tuple = ("hello", 42, "world", [3,6,9]);

    println!("First element is {}", tuple.0);
    println!("Second element is {}", tuple.1);
    println!("Third element is {}", tuple.2);
    let mut counter = 0;
    for x in &tuple.3 {
        println!("Element {} of the fourth element is {}", counter, x);
        counter += 1;
    }
}