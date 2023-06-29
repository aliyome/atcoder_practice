use proconio::input;

fn main() {
    input! {
      d: usize,
      n: usize,
      lrh: [(usize, usize, usize); n],
    }

    let mut days = vec![24; d + 1];
    for &(l, r, h) in &lrh {
        for i in l..=r {
            days[i] = days[i].min(h);
        }
    }

    let mut ans = 0;
    for i in 1..=d {
        ans += days[i];
    }

    println!("{}", ans);
}
