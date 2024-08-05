fn main() {
    proconio::input! {
        x: i32,
    }
    println!("{}", relu(x))
}

fn relu(x: i32) -> i32 {
    return if x >= 0 { x } else { 0 };
}
