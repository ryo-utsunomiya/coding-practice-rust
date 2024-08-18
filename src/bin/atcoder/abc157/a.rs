use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    if n % 2 == 0 {
        println!("{}", n / 2);
    } else {
        println!("{}", n / 2 + 1);
    }
}
