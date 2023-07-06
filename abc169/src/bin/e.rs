use proconio::input;

fn main() {
    input! {
        n: usize,
        ab:[(usize, usize); n]
    };

    let mut ab_asc_by_a = ab.clone();
    let mut ab_desc_by_b = ab.clone();
    ab_asc_by_a.sort_by(|a, b| a.0.cmp(&b.0));
    ab_desc_by_b.sort_by(|a, b| b.1.cmp(&a.1));

    // 1-indexed
    ab_asc_by_a.insert(0, (0, 0));
    ab_desc_by_b.insert(0, (0, 0));

    if n % 2 == 0 {
        // 偶数の場合は平均で中央値を求める
        let mid = n / 2;
        let a = ab_asc_by_a[mid].0 + ab_asc_by_a[mid + 1].0;
        let b = ab_desc_by_b[mid].1 + ab_desc_by_b[mid + 1].1;
        println!("{}", b - a + 1);
    } else {
        // 奇数の場合は中央値を求める
        let mid = (n + 1) / 2;
        let a = ab_asc_by_a[mid].0;
        let b = ab_desc_by_b[mid].1;
        println!("{}", b - a + 1);
    }
}
