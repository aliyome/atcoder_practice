use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut minus = 0i64;
    let mut min = 10i64.pow(8);
    for i in 0..n {
        if a[i] < 0 {
            minus += 1;
        }
        min = min.min(a[i].abs());
    }

    let total = a.iter().map(|x| x.abs()).sum::<i64>();

    if minus % 2 == 0 {
        // すべて正にできる
        println!("{}", total);
    } else {
        // ひとつだけ正にできない
        println!("{}", total - min * 2);
    }
}
