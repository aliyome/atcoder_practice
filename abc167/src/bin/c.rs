use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        c: [(usize, [usize; m]); n]
    };

    let mut ans = std::usize::MAX;
    for i in 0..1 << n {
        let mut status = vec![0; m];
        let mut cost = 0;
        for j in 0..n {
            if i & 1 << j == 0 {
                continue;
            }
            for k in 0..m {
                status[k] += c[j].1[k];
            }
            cost += c[j].0;
        }

        let mut ok = true;
        for k in 0..m {
            if status[k] < x {
                ok = false;
                break;
            }
        }

        if ok {
            ans = ans.min(cost);
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
