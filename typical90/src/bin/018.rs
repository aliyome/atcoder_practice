use proconio::input;

// TODO: 仰角・角度・三平方の定理は苦手…

fn main() {
    input! {
        t: usize, // 10^9
        (l, x, y): (usize, usize, usize), // 10^9
        q: usize, // 1000
        e: [usize; q], // e < t
    }

    for e in e {
        // 角度を求める
        let rad = 2.0 * std::f64::consts::PI * e as f64 / t as f64;
        // let deg = rad * 180.0 / std::f64::consts::PI;
        // y座標を求める
        let ey = -(l as f64 / 2.0) * rad.sin();
        // z座標を求める
        let ez = -(l as f64 / 2.0) * rad.cos() + (l as f64 / 2.0);
        // (0, ey, 0) から (x, y, 0) までの距離を求める
        let a = ((x as f64).powi(2) + (y as f64 - ey).powi(2)).sqrt();
        // 俯角を求める
        let rad = ez.atan2(a as f64);
        // 弧度法に変換
        let ans = rad * 180.0 / std::f64::consts::PI;

        println!("{}", ans);
        // println!(
        //     "rad: {}, deg: {}, ey: {}, ez: {}, a: {}",
        //     rad, deg, ey, ez, a
        // );
    }
}
