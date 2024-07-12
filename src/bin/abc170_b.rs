fn main() {
    proconio::input! {
        x: i32,
        y: i32,
    }
    let mut ans = false;
    for num_two_leg_animals in 0..x+1 {
        if num_two_leg_animals * 2 + (x - num_two_leg_animals) * 4 == y {
            ans = true;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
