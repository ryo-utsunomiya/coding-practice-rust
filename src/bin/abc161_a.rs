fn main() {
    proconio::input! {
        mut a: i32,
        mut b: i32,
        mut c: i32,
    }
    let mut tmp: i32;

    // AとBを入れ替える
    tmp = a;
    a = b;
    b = tmp;

    // AとCを入れ替える
    tmp = a;
    a = c;
    c = tmp;

    println!("{} {} {}", a, b, c);
}
