fn main() {
    proconio::input! {
        n: String,
    }

    let mut sum = 0;
    for c in n.chars() {
        sum = sum + c.to_digit(10).unwrap()
    }

    println!("{}", if sum % 9 == 0 { "Yes" } else { "No" });
}
