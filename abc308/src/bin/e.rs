use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut mex_list = vec![];
    let mut e = vec![];
    let mut x = vec![];
    for i in (0..n).rev() {
        if s[i] == 'M' {
            mex_list.push((i, e.clone(), x.clone()));
        } else if s[i] == 'E' {
            e.push(i);
        } else if s[i] == 'X' {
            x.push(i);
        }
    }

    let get_mex = |a: &[usize]| {
        let mut mex = 0;
        for &x in a {
            if x == mex {
                mex += 1;
            }
        }
        mex
    };

    let mut ans = 0;
    for (i, js, ks) in mex_list {
        for &j in &js {
            if j < i {
                continue;
            }
            for &k in &ks {
                if k < j {
                    continue;
                }
                let mut aa = [a[i], a[j], a[k]];
                aa.sort();
                let mex = get_mex(&aa);
                ans += mex;
                // println!("{} {} {} {} {}", i, j, k, mex, ans);
            }
        }
    }

    println!("{}", ans);
}
