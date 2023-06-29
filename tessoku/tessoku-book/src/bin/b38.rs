use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      s: Chars
    }

    let mut ans = vec![0; n + 2];
    ans[0] = 1;
    for i in 0..n - 1 {
        let idx = i + 1;
        if s[i] == 'A' {
            ans[idx] = ans[idx - 1] + 1;
        } else {
            ans[idx] = 1;
        }
    }
    // println!("{:?}", ans);
    ans[n] = if s[n - 2] == 'A' { 1 } else { 0 };
    for i in (0..n - 1).rev() {
        let idx = i;
        if s[i] == 'B' {
            ans[idx] = ans[idx].max(ans[idx + 1] + 1);
        } else {
            ans[idx] = ans[idx].max(1);
        }
    }
    // println!("{:?}", ans);

    println!("{}", ans[0..n].iter().sum::<usize>());
}
