use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      s: Chars
    }

    let mut t = vec![' '; n];
    let mut l = 0;
    let mut r = n - 1;

    while l < r {
        for i in 0..3 {
            t[l + i] = s[l];
        }
        l += 1;
        // println!("{:?}", t);
        if s == t {
            println!("Yes");
            return;
        }

        for i in 0..3 {
            t[r - i] = s[r];
        }
        r -= 1;
        // println!("{:?}", t);
        if s == t {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
