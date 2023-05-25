use proconio::input;

fn main() {
    input! {
        (n, m): (usize,usize),
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    };

    let mut ans = 0;
    for i in 0..1 << k {
        let mut balls = vec![0; n + 1];
        for j in 0..k {
            if i & (1 << j) == 0 {
                // left
                balls[cd[j].0] += 1;
            } else {
                // right
                balls[cd[j].1] += 1;
            }
        }
        let mut temp = 0;
        for &(a, b) in &ab {
            if balls[a] > 0 && balls[b] > 0 {
                temp += 1;
            }
        }
        ans = ans.max(temp);
    }

    println!("{}", ans);
}
