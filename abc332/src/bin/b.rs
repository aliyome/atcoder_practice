use proconio::input;

fn main() {
    input! {
        k: i32,
        g: i32,
        m: i32,
    }

    let mut glass = 0; // グラスに入っている水の量
    let mut mug = 0; // マグに入っている水の量

    for _ in 0..k {
        if glass == g {
            // グラスが満杯の場合、水を捨てる
            glass = 0;
        } else if mug == 0 {
            // マグが空の場合、水を満たす
            mug = m;
        } else {
            // マグからグラスへ水を移す
            let transfer = g - glass;
            if mug >= transfer {
                glass += transfer;
                mug -= transfer;
            } else {
                glass += mug;
                mug = 0;
            }
        }
    }

    println!("{} {}", glass, mug); // 最終的な水の量を出力
}
