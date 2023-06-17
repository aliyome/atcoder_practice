use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64
    };

    let g: f64 = 1.0;

    // 時刻の計算
    let calc = |n: usize| a / (g + n as f64).sqrt() + n as f64 * b;

    // 三分探索
    let mut l = 0usize;
    let mut r = 10usize.pow(12);
    while l + 3 <= r {
        let c1 = (l * 2 + r) / 3;
        let c2 = (l + r * 2) / 3;
        if calc(c1) < calc(c2) {
            r = c2;
        } else {
            l = c1;
        }
    }

    // 答えの出力
    let mut ans = std::f64::MAX;
    for i in l..=r {
        ans = ans.min(calc(i));
    }

    println!("{}", ans);
}
