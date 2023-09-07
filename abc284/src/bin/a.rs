use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    for i in (0..n).rev() {
        println!("{}", s[i]);
    }
}
