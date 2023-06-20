use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let mut fib = vec![0; n + 1];
    fib[0] = 0;
    fib[1] = 1;
    fib[2] = 1;
    for i in 3..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
        fib[i] %= 10usize.pow(9) + 7;
    }

    println!("{}", fib[n]);
}
