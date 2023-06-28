use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      k: isize,
      s: Chars
    }

    let on_count = s.iter().filter(|&c| *c == '1').count();

    if (k - on_count as isize).abs() % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
