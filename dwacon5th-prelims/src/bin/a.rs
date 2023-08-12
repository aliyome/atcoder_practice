use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n]
    };

    let avg = a.iter().sum::<f64>() / n as f64;
    let mut min = std::f64::MAX;
    let mut min_i = 0;
    for i in 0..n {
        if (a[i] - avg).abs() < min {
            min = (a[i] - avg).abs();
            min_i = i;
        }
    }
    println!("{}", min_i);
}
