// start doc: rustup doc --std
use std::f64::consts;

fn absolute_import() {
    let x = 2.0 * std::f64::consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();

    println!("abs_difference is: {abs_difference:.10}");
    assert!(abs_difference < 1e-10);
}

fn relative_impport() {
    // dont forget use std::f64::consts; at the top
    let x = 2.0 * consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();

    println!("abs_difference is: {abs_difference:.15}");
    assert!(abs_difference < 1e-10);
}

pub fn main(){
    absolute_import();
    relative_impport();
}