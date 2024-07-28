fn main() {
    proconio::input! {
        n: i32,
        x: [i64; n],
    }

    println!("{}", manhattan_distance(&x));
    println!("{}", euclidean_distance(&x));
    println!("{}", chebyshev_distance(&x));
}

fn manhattan_distance(x: &Vec<i64>) -> i64 {
    x.iter().map(|v| v.abs()).sum()
}

fn euclidean_distance(x: &Vec<i64>) -> f64 {
    x.iter().map(|v| v.abs().pow(2) as f64).sum::<f64>().sqrt()
}

fn chebyshev_distance(x: &Vec<i64>) -> i64 {
    x.iter().map(|v| v.abs()).max().unwrap()
}
