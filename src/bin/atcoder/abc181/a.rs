fn main() {
    proconio::input! {
        n: i32,
    }

    let ans = if n % 2 == 0 { "White" } else { "Black" };
    println!("{}", ans);
}
