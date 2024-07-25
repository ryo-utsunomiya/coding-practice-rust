use std::f64::consts::PI;

fn main() {
    proconio::input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    // 時針は12時間で360度回転するため、1時間あたりの回転角度は30度（1分あたり0.5度）
    // 分針は1時間で360度回転するため、1分あたりの回転角度は6度
    // これらに PI / 180 をかけることでラジアンに変換する
    let hour = (30.0 * h + 0.5 * m) * PI / 180.0;
    let minute = 6.0 * m * PI / 180.0;

    // 時針と分針の角度
    let theta = (hour - minute).abs();

    // 余弦定理: c^2 = a^2 + b^2 - 2 * a * b * cos(theta)
    let distance = (a.powi(2) + b.powi(2) - 2.0 * a * b * theta.cos()).sqrt();

    println!("{}", distance)
}
