fn main() {
    proconio::input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    println!("{}", calculate_reflection_point(sx, sy, gx, gy))
}

// ターゲット座標(gx, gy)の反対側の座標(gx, -gy)に向かって進む直線をかけばよい
// 反対側の座標(gx, -gy)から出発点(sx, sy)への直線は以下の式で表される
// y = (-gy - sy) / (gx - sx) * (x - sx) + sy
// この直線とx軸との交点を求めるため y=0 を代入すると、
// x = sx + (sy * (gx - sx)) / (gy + sy)
fn calculate_reflection_point(sx: f64, sy: f64, gx: f64, gy: f64) -> f64 {
    sx + (sy * (gx - sx)) / (gy + sy)
}
