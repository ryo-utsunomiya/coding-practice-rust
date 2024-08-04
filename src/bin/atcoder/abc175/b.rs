use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (l[i], l[j], l[k]);
                // 三辺がすべて異なる
                if (a != b && b != c && c != a)
                    // 三角形が成立する
                    && (a + b > c && b + c > a && c + a > b)
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
