use proconio::input;

fn main() {
    input! {
        n: f32,
    }

    let x = (1.08 * n).floor() as i32;
    let list_price = 206;
    if x == list_price {
        println!("so-so")
    } else if x < list_price {
        println!("Yay!")
    } else {
        println!(":(")
    }
}
