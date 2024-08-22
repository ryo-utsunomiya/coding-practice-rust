use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    let l = a + b;
    let r = c + d;
    if l > r {
        println!("Left");
    } else if l == r {
        println!("Balanced");
    } else {
        println!("Right");
    }
}
