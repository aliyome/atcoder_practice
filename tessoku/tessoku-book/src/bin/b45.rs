use proconio::input;

fn main() {
    input! {
      a: isize,
      b: isize,
      c: isize
    }

    if a + b + c == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
