use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    };

    if m > n {
        println!("0");
        return;
    }

    println!("{}", (n - m) / p + 1);
}
