use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }

    let uniq: HashSet<i32> = d.into_iter().collect();
    println!("{}", uniq.len());
}
