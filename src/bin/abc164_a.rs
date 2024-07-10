fn main() {
    proconio::input! {
        s: i32,
        w: i32,
    }
    println!("{}", if w >= s { "unsafe" } else { "safe" })
}
