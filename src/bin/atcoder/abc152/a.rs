fn main() {
    proconio::input! {
        n: i32,
        m: i32,
    }
    println!("{}", if n == m { "Yes" } else { "No" })
}
