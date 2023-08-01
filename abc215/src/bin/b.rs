use proconio::input;

fn main() {
    input! {
        n: f64,
    };

    println!("{}", n.log2().floor());
}
