use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize,
    };

    println!("{}", (p + q).min(q + r).min(r + p));
}
