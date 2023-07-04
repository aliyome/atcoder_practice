use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    };

    // sums[i][j] := i番目以降にjが出現する回数 (jはa[i]に200の下駄を履かせたもの)
    let mut sums = vec![vec![0; 400]; n + 1];
    for i in 0..n {
        let idx = n - i - 1;
        sums[idx] = sums[idx + 1].clone();
        sums[idx][(a[idx] + 200) as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..400 {
            if sums[i][j] == 0 {
                continue;
            }
            ans += ((a[i] + 200) - j as isize).pow(2) * sums[i][j];
        }
    }

    println!("{}", ans);
}
