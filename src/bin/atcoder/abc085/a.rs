use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!("{}", s.replace("2017/01/", "2018/01/"));
}
