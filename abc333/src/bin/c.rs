use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    // repunitの和として表現できる数を見つける
    let mut count = 0;
    'outer: for i in 0.. {
        for j in 0..=i {
            for k in 0..=j {
                // 3つのrepunitの和を計算
                let sum = repunit(i) + repunit(j) + repunit(k);
                count += 1;

                // N番目の数を見つけた場合
                if count == n {
                    result = sum;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", result);
}

// repunitを計算する関数
fn repunit(len: usize) -> usize {
    let mut repunit = 0;
    for _ in 0..=len {
        repunit = repunit * 10 + 1;
    }
    repunit
}
