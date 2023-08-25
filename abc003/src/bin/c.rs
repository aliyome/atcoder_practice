use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [usize; n]
    };

    r.sort();
    r.reverse();

    let mut ans = 0.0;
    for i in (0..k).rev() {
        let r = r[i] as f64;
        ans = (ans + r) / 2.0;
    }

    println!("{}", ans);
}
