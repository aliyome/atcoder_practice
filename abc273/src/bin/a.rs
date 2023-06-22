use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 0 {
        println!("1");
    } else {
        println!("{}", f(n));
    }
}

fn f(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * f(n - 1)
    }
}
