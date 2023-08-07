use proconio::input;

// TODO: 仰角・角度・三平方の定理は苦手…

fn main() {
    input! {
        t: usize, // 10^9
        (l, x, y): (usize, usize, usize), // 10^9
        q: usize, // 1000
        e: [usize; q], // e < t
    }

    for &e in &e {
        let rad = 2.0 * std::f64::consts::PI * (e as f64 / t as f64);
        let yy = -(l as f64 / 2.0) * rad.sin();
        let zz = (l as f64 / 2.0) - (l as f64 / 2.0) * rad.cos();
        let u = ((x as f64).powf(2.0) + (y as f64 - yy).powf(2.0)).sqrt();
        let theta = zz.atan2(u).to_degrees();
        println!("{}", theta);
    }
}
