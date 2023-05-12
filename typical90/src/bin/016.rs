use proconio::input;

fn main() {
    input! {
        n: usize,
        (a, b, c): (usize, usize, usize)
    }

    // 全探索
    let mut min = 10usize.pow(4);
    for i in 0..10usize.pow(4) {
        if i * a > n {
            break;
        }
        for j in 0..10usize.pow(4) {
            if i + j > 9999 || i * a + j * b > n {
                break;
            }
            // AとBの枚数を固定したらCの枚数は一意に定まる
            if (n - (i * a + j * b)) % c == 0 {
                let k = (n - (i * a + j * b)) / c;
                min = min.min(i + j + k);
            }
        }
    }

    println!("{}", min);
}
