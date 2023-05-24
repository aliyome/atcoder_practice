use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut curr = vec![0; n + 1];
    let mut sum = vec![0; n + 1];
    let mut tmp_max = -100_000_000;
    let mut max = 0;
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
        tmp_max = tmp_max.max(sum[i + 1]);
        curr[i + 1] = curr[i] + sum[i + 1];
        max = max.max(curr[i] + tmp_max);
    }

    // println!("{:?}", sum);
    // println!("{:?}", curr);

    println!("{}", max);
}
