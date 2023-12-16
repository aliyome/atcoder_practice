use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    // 頂点間の距離の種類を判断する関数
    fn distance_type(s: &str) -> i32 {
        let to_int = |c: char| -> i32 { c as i32 - 'A' as i32 };
        let a = to_int(s.chars().nth(0).unwrap());
        let b = to_int(s.chars().nth(1).unwrap());
        let diff = (a - b).abs();

        // 頂点間の距離が1または4（隣接する頂点）なら1、そうでなければ2（2つ離れた頂点）
        if diff == 1 || diff == 4 {
            1
        } else {
            2
        }
    }

    // 両方のペアの距離の種類が同じかどうかを判断する
    if distance_type(&s) == distance_type(&t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
