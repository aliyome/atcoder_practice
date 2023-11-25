use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize, usize); q],
    }

    let mut freq = vec![0; n + 1];
    let mut mex = 0;

    // 初期配列の頻度を計算
    for &value in &a {
        if value <= n {
            freq[value] += 1;
        }
    }

    // 初期のmexを計算
    while mex <= n && freq[mex] > 0 {
        mex += 1;
    }

    // クエリを処理
    for (i, x) in queries {
        let index = i - 1; // 0-indexedに変換
        let old_value = a[index];
        a[index] = x;

        // 古い値の頻度を減らす
        if old_value <= n {
            freq[old_value] -= 1;
            if freq[old_value] == 0 && old_value < mex {
                mex = old_value;
            }
        }

        // 新しい値の頻度を増やす
        if x <= n {
            freq[x] += 1;
            while mex <= n && freq[mex] > 0 {
                mex += 1;
            }
        }

        println!("{}", mex);
    }
}
