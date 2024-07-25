fn main() {
    proconio::input! {
        n: i32,
        a: i32,
    }

    let d = n % 500;
    let ans = if d <= a { "Yes" } else { "No" };
    println!("{}", ans);
}
