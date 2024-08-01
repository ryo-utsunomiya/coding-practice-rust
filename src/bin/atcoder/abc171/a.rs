use proconio::input;

fn main() {
    input! {
        a: char,
    }

    if 'A' <= a && a <= 'Z' {
        println!("A");
    } else {
        println!("a");
    }
}
