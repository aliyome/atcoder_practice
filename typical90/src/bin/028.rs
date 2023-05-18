use proconio::input;

fn main() {
    input! {
        n: usize,
        squares: [(usize, usize, usize, usize); n]
    }

    // いもす法 O(N + HW)
    let mut imos = vec![vec![0; 1002]; 1002];
    // 重みの加算 O(N)
    for i in 0..n {
        let (lx, ly, rx, ry) = squares[i];
        imos[ly][lx] += 1;
        imos[ry][rx] += 1;
        imos[ry][lx] -= 1;
        imos[ly][rx] -= 1;
    }
    // 横方向の累積和 O(HW)
    for y in 0..1002 {
        for x in 1..1002 {
            imos[y][x] += imos[y][x - 1];
        }
    }
    // 縦方向の累積和 O(HW)
    for x in 0..1002 {
        for y in 1..1002 {
            imos[y][x] += imos[y - 1][x];
        }
    }
    // 重なっている正方形の数を数える O(HW)
    let mut ans = vec![0; n + 1];
    for y in 0..1002 {
        for x in 0..1002 {
            ans[imos[y][x] as usize] += 1;
        }
    }
    // 出力 O(N)
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
