fn main() {
    proconio::input! {
        d: i32,
        t: i32,
        s: i32,
    }
    if s * t >= d {
        println!("Yes")
    } else {
        println!("No")
    }
}
