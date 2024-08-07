use proconio::input;

fn main() {
    input! {
        a: char,
    }

    let ans = match a {
        'A'..='Z' => 'A',
        'a'..='z' => 'a',
        _ => unreachable!(),
    };
    println!("{}", ans);
}
