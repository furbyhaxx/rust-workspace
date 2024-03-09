
/// Good old hello world
fn hello() {
    println!("Hello, world");
}

/// simple (immutable) function definition
fn let1() {
    let answer = 42;
    println!("Hello {}", answer);
}

/// basic assertion
fn let2() {
    let answer = 42;
    assert_eq!(answer, 42);
}

/// basic for loop with range (non inclusive)
fn loop_for1() {
    for i in 0..5 {
        println!("Hello {}", i);
    }
}

/// for loop with condition
fn loop_for2(){
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
}

/// loop_for2 simplified
fn loop_for3() {
    for i in 0..5 {
        let result = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", result, i);
    }
}

/// simple addition
/// mutable variables
fn add1() {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}
/// addition with float and type casting
/// println! format float precision
fn add2() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {:.5}", sum);
}

/// simple function calling
/// order of function do not matter
fn fun1() {
    let x = 4.0;
    let res = sqr(x);
    println!("srq of {:.2} is {:.2}", x, res);
}

/// simple function which sqaures the input x
fn sqr(x: f64) -> f64 {
    x * x // no semicolon at end
    // this is also valid but unneeded
    // return can be used to early exit functions
    // return x * x
}

/// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

/// ensure the number always falls in the given range
fn clamp(x: f64, lower: f64, upper: f64) -> f64 {
    if x < lower {
        lower
    } else if x > upper {
        upper
    } else {
        x
    }
}

/// recurse function
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

/// function parameters as reference
/// prevents copying of the variable
/// useful for large items
/// & = borrow = reference = pointer
/// * = dereference
fn by_ref(x: &i32) -> i32 {
    *x + 1
}
fn by_ref2(x: &i32) -> i32{
    x + 1
}

// fn by_ref3(x: &mut i32) -> i32{
//     &(x + 1)
// }

fn fun_by_ref() {
    let mut i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);

    println!("by_ref: {} {}", res1, res2);
}

fn fun_by_ref2() {
    let i = 10;
    let res1 = by_ref2(&i);
    let res2 = by_ref2(&41);

    println!("by_ref2: {} {}", res1, res2);
}

// fn fun_by_ref3() {
//     let i = 10;
//     by_ref2(&i);
//     println!("by_ref3: {}", i);
// }

fn fun_by_ref4() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {:.2}", res);
}
/// no return type as it's modifying x and not returning
fn modifies(x: &mut f64) {
    *x = 1.0;
}

/// main func
/// pub to be callable outside of this module
pub fn main() {
    // hello();
    // let1();
    // let2();
    // loop_for1();
    // loop_for2();
    // loop_for3();
    // add1();
    // add2();
    // fun1();

    fun_by_ref();
    fun_by_ref2();
    // fun_by_ref3();
    fun_by_ref4();
}