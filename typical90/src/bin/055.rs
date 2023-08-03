use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }

    let mul = |a, b| {
        let a = a % p;
        let b = b % p;
        let c = a * b;
        c % p
    };

    let mut ans = 0usize;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        // a*b*c*d*e % p == q の数を数える
                        // 単純に a*b*c*d*e % p == q としてしまうとオーバーフローするので
                        // 掛け算のたびに mod する
                        if mul(mul(mul(mul(a[i], a[j]), a[k]), a[l]), a[m]) == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
