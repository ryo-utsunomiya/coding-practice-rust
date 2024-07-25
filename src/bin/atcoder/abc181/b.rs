fn main() {
    proconio::input! {
        n: usize,
        v: [(i64, i64); n],
    }

    let mut ans = 0;

    for i in 0..n {
        let a = v[i].0;
        let b = v[i].1;
        for d in a..b + 1 {
            ans += d;
        }
    }

    println!("{}", ans);
}
