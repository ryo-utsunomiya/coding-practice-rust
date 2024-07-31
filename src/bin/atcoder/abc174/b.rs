fn main() {
    proconio::input! {
        n: i32,
        d: f64,
        p: [(f64, f64); n],
    }

    println!("{}", p.iter().filter(|&&(x, y)| x.hypot(y) <= d).count());
}
