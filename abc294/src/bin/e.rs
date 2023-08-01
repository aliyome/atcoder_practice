use proconio::input;

fn main() {
    input! {
        l: usize,
        n1: usize,
        n2: usize,
        mut v1l: [(usize, usize); n1],
        mut v2l: [(usize, usize); n2],
    };

    v1l.push((0, 0));
    v2l.push((0, 0));

    // インデックス
    let mut i1 = 0;
    let mut i2 = 0;
    // 開始位置
    let mut s1 = 0;
    let mut s2 = 0;
    // 重複部分の数
    let mut ans = 0;

    while !(i1 >= n1 || i2 >= n2) {
        let (v1, l1) = v1l[i1];
        let (v2, l2) = v2l[i2];
        if v1 == v2 {
            // 重複部分の数を数える
            if s1 <= s2 && s2 <= s1 + l1 && s1 + l1 <= s2 + l2 {
                ans += s1 + l1 - s2;
            } else if s2 <= s1 && s1 <= s2 + l2 && s2 + l2 <= s1 + l1 {
                ans += s2 + l2 - s1;
            } else if s1 <= s2 && s2 + l2 <= s1 + l1 {
                ans += l2;
            } else if s2 <= s1 && s1 + l1 <= s2 + l2 {
                ans += l1;
            }
        }
        if s1 + l1 < s2 + l2 {
            s1 += l1;
            i1 += 1;
        } else {
            s2 += l2;
            i2 += 1;
        }
    }
    println!("{}", ans);
}
