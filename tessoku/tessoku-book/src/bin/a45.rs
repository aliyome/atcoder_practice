use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      c: char,
      a: Chars
    }

    let mut score = 0;
    for &a in &a {
        if a == 'W' {
            score += 0;
        } else if a == 'B' {
            score += 1;
        } else {
            score += 2;
        }
    }

    let mut res = 'W';
    if score % 3 == 0 {
        res = 'W';
    } else if score % 3 == 1 {
        res = 'B';
    } else {
        res = 'R';
    }

    if c == res {
        println!("Yes");
    } else {
        println!("No");
    }
}
