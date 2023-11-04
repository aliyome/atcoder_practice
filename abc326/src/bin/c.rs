use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }

    a.sort();

    let mut r = 0;
    let mut ans = 0;
    for l in 0..n {
        for next_r in r..n {
            if a[next_r] < a[l] + m {
                r = next_r;
            } else {
                break;
            }
        }
        ans = ans.max(r - l + 1);
    }

    println!("{}", ans);
}
