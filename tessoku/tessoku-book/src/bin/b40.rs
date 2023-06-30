use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if (a[i] + a[j]) % 100 == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
