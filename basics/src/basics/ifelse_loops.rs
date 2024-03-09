pub fn main() {
    println!("If/Else Statements");

    let n = 3;

    if n < 10 {
        println!("{} is less than 10", n);
    } else {
        println!("{} is greater than or equal to 10", n);
    }

    //--------------------------------------------------------------------------//
    println!("Loop Statements");

    let mut x = 0;

    loop {
        x += 1;

        if x >= 30 {
            println!("x is now greater than or equal to 30 so we break the loop");
            break;
        }

        if x % 3 != 0 {
            continue; // The print statment following this if statement will not be printed
        }

        println!("{} is divisible by 3", x);
    }

    //--------------------------------------------------------------------------//
    println!("While Statements");

    let mut y = 0;

    while y <= 30 {
        y += 1;

        if y % 3 == 0 {
            println!("{} is divisible by 3", y);
        }
    }


    //--------------------------------------------------------------------------//
    println!("For-In Statements");

    for z in 1..31 {
        if z % 3 == 0 {
            println!("{} is divisible by 3", z);
        }
    }
}