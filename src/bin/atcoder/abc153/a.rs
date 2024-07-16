fn main() {
    proconio::input! {
        h: i32,
        a: i32,
    }
    let ans = if h % a == 0 {
        h / a
    } else {
        h / a + 1
    };
    println!("{}", ans);
}
