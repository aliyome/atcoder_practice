use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    for &i in &[1, 2, 3] {
        if i != a && i != b {
            println!("{}", i);
            return;
        }
    }
}
