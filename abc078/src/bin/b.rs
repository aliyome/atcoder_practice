use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };

    for i in 0..1000000 {
        if i * y + (i + 1) * z > x {
            println!("{}", i - 1);
            return;
        }
    }
}
