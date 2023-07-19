use proconio::input;

fn main() {
    input! {
        n: usize, // <= 2000
        tlr: [(usize, usize, usize); n] // l,r <= 10^9
    };

    let normalize = |t, l, r| {
        let l = l as f64;
        let r = r as f64;
        if t == 1 {
            (l, r)
        } else if t == 2 {
            (l, r - 0.5)
        } else if t == 3 {
            (l + 0.5, r)
        } else {
            (l + 0.5, r - 0.5)
        }
    };

    let mut ans = 0usize;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if i == j {
                continue;
            }
            let (ti, li, ri) = tlr[i];
            let (tj, lj, rj) = tlr[j];
            let (li, ri) = normalize(ti, li, ri);
            let (lj, rj) = normalize(tj, lj, rj);
            if li <= lj && lj <= ri
                || li <= rj && rj <= ri
                || lj <= li && li <= rj
                || lj <= ri && ri <= rj
            {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
