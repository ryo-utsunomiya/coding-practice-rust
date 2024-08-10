use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = true;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i].is_uppercase() {
            ans = false;
            break;
        } else if i % 2 != 0 && s[i].is_lowercase() {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
