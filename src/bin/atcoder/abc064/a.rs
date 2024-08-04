use proconio::input;

fn main() {
    input! {
        r: i32,
        g: i32,
        b: i32,
    }

    if (r * 100 + g * 10 + b) % 4 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
