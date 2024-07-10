fn main() {
    proconio::input! {
        k: i32,
        a: i32,
        b: i32,
    }

    let mut ans = false;
    for i in a..b+1 {
        if i % k == 0 {
            ans = true;
            break;
        }
    }
    println!("{}", if ans { "OK" } else { "NG" })
}
