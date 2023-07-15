use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n]
    };

    let side = d.iter().min().unwrap();

    println!("{}", p.min(q + side));
}
