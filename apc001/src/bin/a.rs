use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    if x % y == 0 {
        println!("-1");
    } else {
        println!("{}", x);
    }
}
