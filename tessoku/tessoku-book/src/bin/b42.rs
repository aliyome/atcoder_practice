use proconio::input;

fn main() {
    input! {
      n: usize,
      ab: [(isize, isize); n],
    }

    // 0: +a +b
    // 1: +a -b
    // 2: -a +b
    // 3: -a -b
    let mut memo = vec![(0isize, 0isize); 4];
    for &(a, b) in &ab {
        if a + b > 0 {
            memo[0].0 += a;
            memo[0].1 += b;
        }
        if a - b > 0 {
            memo[1].0 += a;
            memo[1].1 += b;
        }
        if b - a > 0 {
            memo[2].0 += a;
            memo[2].1 += b;
        }
        if a + b < 0 {
            memo[3].0 += a;
            memo[3].1 += b;
        }
    }

    let mut ans = std::isize::MIN;
    for i in 0..4 {
        ans = ans.max(memo[i].0.abs() + memo[i].1.abs());
    }

    println!("{}", ans);
}
