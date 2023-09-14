use proconio::input;

fn main() {
    input! {
        m1: usize,
        d1: usize,
        m2: usize,
        d2: usize,
    };

    if m1 != m2 {
        println!("1");
    } else {
        println!("0");
    }
}
