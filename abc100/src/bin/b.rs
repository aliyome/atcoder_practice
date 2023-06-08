use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    };

    if d == 0 {
        println!("{}", n);
    } else {
        println!("{}", 100usize.pow(d as u32) * n);
    }
}
