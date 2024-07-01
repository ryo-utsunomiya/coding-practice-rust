fn main() {
    proconio::input! {
        n: i32,
        x: i32,
        t: i32,
    }
    let mut ans = n / x * t;
    if n % x != 0 {
        ans += t;
    }
    println!("{}", ans)
}
