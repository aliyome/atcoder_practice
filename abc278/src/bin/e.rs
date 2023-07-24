use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[usize; w]; h]
    };

    // 出現回数を数える
    let mut histogram = vec![0; n + 1];
    for i in 0..h {
        for j in 0..w {
            histogram[a[i][j]] += 1;
        }
    }

    let calc = |a: &Vec<Vec<usize>>, si: usize, sj: usize| {
        let mut hist = histogram.clone();
        for i in si..si + hh {
            for j in sj..sj + ww {
                hist[a[i][j]] -= 1;
            }
        }
        let mut sum = 0;
        for i in 0..=n {
            if hist[i] > 0 {
                sum += 1;
            }
        }
        sum
    };

    // ウィンドウをスライドして全探索
    for i in 0..=h - hh {
        for j in 0..=w - ww {
            print!("{} ", calc(&a, i, j));
        }
        println!();
    }
}
