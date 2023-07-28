use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut ans = 0;
    // O(N^2) 100^2 = 10000
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            ans = ans.max(a[i] + a[j]);
        }
    }

    println!("{}", ans);
}
