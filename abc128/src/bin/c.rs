use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    // M個の電球, N個のスイッチ

    // links[i] := i番目の電球に繋がっているスイッチを表すビット列
    let mut links = vec![0usize; m];
    for i in 0..m {
        input! {
            k: usize,
            s: [usize; k]
        }
        for j in 0..k {
            links[i] |= 1 << (s[j] - 1);
        }
    }

    input! {
        p: [usize; m]
    }

    // ビット全探索
    // bit はスイッチのon/offの組み合わせを表す
    let mut ans = 0;
    for bit in 0usize..1 << n {
        let mut ok = true;
        for i in 0..m {
            let switch = links[i];
            let p = p[i];
            if ((bit & switch).count_ones() % 2) as usize == p {
                // 点灯する
                continue;
            } else {
                // 点灯しない
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
