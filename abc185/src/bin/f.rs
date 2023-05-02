use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    };

    // 愚直に実装。TLE
    for (t, x, y) in txy {
        if t == 1 {
            a[x - 1] ^= y;
        } else {
            let mut ans = a[x - 1];
            for i in x..y {
                ans ^= a[i];
            }
            println!("{}", ans);
        }
    }
}
