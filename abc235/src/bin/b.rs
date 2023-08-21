use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    };

    for i in 0..n - 1 {
        if h[i] >= h[i + 1] {
            println!("{}", h[i]);
            return;
        }
    }

    println!("{}", h[n - 1]);
}
