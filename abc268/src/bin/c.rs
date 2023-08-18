use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    };

    // sum[i] := 反時計回りに i 回転したときの合計人数
    let mut sum = vec![0; n];

    for i in 0..n {
        let diff = (n + p[i] - i) % n;
        let a = (n + diff - 1) % n;
        let b = diff;
        let c = (diff + 1) % n;
        sum[a] += 1;
        sum[b] += 1;
        sum[c] += 1;
    }

    println!("{}", sum.iter().max().unwrap());
}
