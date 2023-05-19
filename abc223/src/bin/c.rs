use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    };

    // 導火線が燃え尽きるまでにかかる時間を S とすると
    // 両端から導火線に🔥をつけた場合、燃え尽きるまでにかかる時間は S/2
    // -> S を求めてから S/2 を求めて、その後 S/2 までに燃え尽きる導火線の長さを求める

    let mut s = 0f64;
    for (a, b) in &ab {
        s += *a / *b;
    }

    // 経過時間
    let mut t = 0f64;
    let mut l = 0f64;
    for (a, b) in &ab {
        let dt = *a / *b;
        if t + dt < s / 2f64 {
            // S/2 以下の場合は単純加算
            t += *a / *b;
            l += *a;
            continue;
        } else {
            // S/2 を超える場合は、S/2 までに燃え尽きる導火線の長さを求める
            let rest_time = s / 2f64 - t;
            let ans = l + rest_time * *b;
            println!("{}", ans);
            return;
        }
    }
}
