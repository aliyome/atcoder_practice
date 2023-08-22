use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    // 2n行の列数
    let c = if w % 2 == 0 { w / 2 } else { w / 2 + 1 };
    // 2n+1行の列数
    let d = w / 2;

    // 2nの行数
    let a = if h % 2 == 0 { h / 2 } else { h / 2 + 1 };
    // 2n+1の行数
    let b = h / 2;

    println!("{}", a * c + b * d);
}
