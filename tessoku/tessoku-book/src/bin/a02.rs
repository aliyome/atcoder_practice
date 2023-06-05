use proconio::input;

fn main() {
    input! {
      n: usize,
      x: usize,
      a: [usize; n]
    }

    for a in a {
        if x == a {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
