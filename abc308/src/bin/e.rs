use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut m = vec![vec![]; n + 1];
    let mut e = vec![vec![]; n + 1];
    let mut x = vec![vec![]; n + 1];
    for i in (0..n).rev() {
        m[i] = m[i + 1].clone();
        e[i] = e[i + 1].clone();
        x[i] = x[i + 1].clone();
        if s[i] == 'M' {
            m[i].push(i);
        } else if s[i] == 'E' {
            e[i].push(i);
        } else if s[i] == 'X' {
            x[i].push(i);
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
    for i in 0..n {
        // 新しいMがあったら
        if s[i] == 'M' {
            // println!("{:?} {:?} {:?}", m[i], e[i], x[i]);
            for &j in &e[i] {
                if j < i {
                    continue;
                }
                for &k in &x[i] {
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
    }

    println!("{}", ans);
}
