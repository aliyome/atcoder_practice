use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        px: [[usize; 2]; m]
    };

    let total_time: usize = t.iter().sum();

    for i in 0..m {
        let p = px[i][0] - 1;
        let x = px[i][1];
        let new_total = total_time - t[p] + x;
        println!("{}", new_total);
    }
}
