use proconio::input;
use std::process::exit;

fn main() {
    input! {
        n: i64,
    }

    let mut sum = 0_i64;
    for i in 1..=n {
        sum += i;
        if sum >= n {
            println!("{}", i);
            exit(0);
        }
    }
}
