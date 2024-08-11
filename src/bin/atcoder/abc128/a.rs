use proconio::input;

fn main() {
    input! {
        a: i32,
        p: i32,
    }
    let ans = (a * 3 + p) / 2;
    println!("{}", ans);
}
