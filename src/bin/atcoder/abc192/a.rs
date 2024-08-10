use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    let ans = (x % 100 - 100).abs();
    println!("{}", ans);
}
