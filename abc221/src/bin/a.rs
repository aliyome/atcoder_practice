use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    println!("{}", 32usize.pow(a) / 32usize.pow(b));
}
