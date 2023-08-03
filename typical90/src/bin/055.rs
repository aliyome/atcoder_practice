use proconio::input;

fn main() {
    input! {
        n: usize, // <= 100
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

    // O(N^5) で全探索
    // (100^5) = 10^10 なので間に合わない、は間違い
    // nC5 通りの組み合わせを全探索する
    // = n(n-1)(n-2)(n-3)(n-4)/5! ≒ 1/120 * N^5 通り
    // → 10^10 / 120 = 10^8 なので間に合う
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
