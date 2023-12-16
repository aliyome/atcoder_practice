use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    // 頂点間の距離を比較する関数
    fn is_equal_distance(s: &str, t: &str) -> bool {
        // 頂点のラベルを数値に変換する
        let to_int = |c: char| -> i32 { c as i32 - 'A' as i32 };

        let s1 = to_int(s.chars().nth(0).unwrap());
        let s2 = to_int(s.chars().nth(1).unwrap());
        let t1 = to_int(t.chars().nth(0).unwrap());
        let t2 = to_int(t.chars().nth(1).unwrap());

        // 正五角形の頂点間の距離は、ラベルの差の絶対値に依存する
        (s1 - s2).abs() % 5 == (t1 - t2).abs() % 5
    }

    // 結果を出力
    if is_equal_distance(&s, &t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
