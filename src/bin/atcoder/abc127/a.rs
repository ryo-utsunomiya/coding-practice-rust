fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }

    let ans = if a >= 13 {
        b
    } else if a >= 6 && a <= 12 {
        b / 2
    } else {
        0
    };
    println!("{}", ans);
}
