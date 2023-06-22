use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut xor = a[0];
    for i in 1..n {
        xor ^= a[i];
    }

    if xor == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
