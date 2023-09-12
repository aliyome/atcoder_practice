use proconio::input;

fn main() {
    input! {
        n: usize, // 2x10^5
        m: usize, // 2x10^5
        d: i64, // 10^18
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();

    let mut ans = -1;
    for &a in &a {
        let i = b.partition_point(|&x| x <= a + d);
        if i >= 1 {
            if (a - b[i - 1]).abs() <= d {
                ans = ans.max(a + b[i - 1]);
            }
        }
        if i < m {
            if (a - b[i]).abs() <= d {
                ans = ans.max(a + b[i]);
            }
        }
        if i + 1 < m {
            if (a - b[i + 1]).abs() <= d {
                ans = ans.max(a + b[i + 1]);
            }
        }
    }

    println!("{}", ans);
}
