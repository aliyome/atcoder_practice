use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    };

    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if ((xy[i].0 - xy[k].0) * (xy[j].1 - xy[k].1)
                    - (xy[j].0 - xy[k].0) * (xy[i].1 - xy[k].1))
                    .abs()
                    > 0
                {
                    ans += 1;
                };
            }
        }
    }

    println!("{}", ans);
}
