use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        _: i32,
        mut x: i32,
        s: String,
    }

    for c in s.chars() {
        if c == 'o' {
            x += 1;
        } else {
            x = max(x - 1, 0);
        }
    }
    println!("{}", x);
}
