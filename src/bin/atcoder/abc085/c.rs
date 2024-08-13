use proconio::input;
use std::process::exit;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    for a in 0..=n {
        for b in 0..=n {
            let c = n - a - b;
            if c >= 0 && a * 10000 + b * 5000 + c * 1000 == y {
                println!("{} {} {}", a, b, c);
                exit(0);
            }
        }
    }

    println!("-1 -1 -1");
}
