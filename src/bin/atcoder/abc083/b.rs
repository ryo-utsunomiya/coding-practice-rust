use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = 0;
    for i in 1..=n {
        let sum = sum_digits(i);
        if a <= sum && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}

fn sum_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut m = n;
    while m > 0 {
        sum += m % 10;
        m /= 10;
    }
    sum
}
