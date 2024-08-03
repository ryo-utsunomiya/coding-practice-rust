use proconio::input;

fn main() {
    input! {
        s: String
    }

    let ans = if s == "RRR" {
        3
    } else if s.contains("RR") {
        2
    } else if s.contains("R") {
        1
    } else {
        0
    };
    println!("{}", ans);
}
