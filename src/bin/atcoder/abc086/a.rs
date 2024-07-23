fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let ans = if a * b % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", ans);
}
