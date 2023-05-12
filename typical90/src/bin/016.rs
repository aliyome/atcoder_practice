use proconio::input;

fn main() {
    input! {
        n: usize,
        (a, b, c): (usize, usize, usize)
    }

    // 全探索 TLE
    let mut min = 10usize.pow(4);
    for i in 0..10usize.pow(4) {
        if i * a > n {
            break;
        }
        for j in 0..10usize.pow(4) {
            if i + j > 9999 || i * a + j * b > n {
                break;
            }
            for k in 0..10usize.pow(4) {
                if i + j + k > 9999 || i * a + j * b + k * c > n {
                    break;
                }
                if a * i + b * j + c * k == n {
                    min = min.min(i + j + k);
                }
            }
        }
    }

    println!("{}", min);
}
