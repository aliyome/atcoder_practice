use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    let ans = (w - 1) * h + (h - 1) * w;
    println!("{}", ans);
}
