use proconio::input;

fn main() {
    input! {
      (n, m, b): (usize, usize, usize),
      a: [usize; n],
      c: [usize; m],
    }

    let mut ans = 0;
    ans += b * n * m;
    ans += a.iter().sum::<usize>() * m;
    ans += c.iter().sum::<usize>() * n;
    println!("{}", ans);
}
