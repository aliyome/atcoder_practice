use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    println!("{}", n);
    for i in 1..n {
        println!("{} {}", i, i + 1);
    }
    println!("{} {}", 1, n);
}
