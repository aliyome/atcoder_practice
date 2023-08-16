use proconio::input;

fn main() {
    input! {
        mut x: [usize; 3],
    };

    let mut ans = 0;
    while !x.iter().all(|&i| i == x[0]) {
        let max = *x.iter().max().unwrap();
        let cnt = x.iter().filter(|&i| *i < max).count();
        if cnt == 2 {
            for i in 0..3 {
                if x[i] != max {
                    x[i] += 1;
                }
            }
        } else {
            for i in 0..3 {
                if x[i] < max {
                    x[i] += 2;
                }
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}
