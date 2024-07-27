use std::cmp::max;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }

    let mut prev_height = 0;
    let mut ans = 0i64;

    for height in a {
        let diff = prev_height - height;
        let adjustment = max(0, diff);
        ans += i64::from(adjustment);
        prev_height = height + adjustment;
    }

    println!("{}", ans);
}
