use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();

    let mut ans = -1;
    for &a_val in &a {
        let idx = match b.binary_search(&(a_val + d + 1)) {
            Ok(i) => i,
            Err(i) => i,
        };
        if idx > 0 {
            let b_val = b[idx - 1];
            if (a_val - b_val).abs() <= d {
                ans = ans.max(a_val + b_val);
            }
        }
    }

    println!("{}", ans);
}
