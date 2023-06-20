use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    // overflow
    // println!("{}", a.pow(b) % 1000000007);

    // TLE
    // let mut ans = 1;
    // for i in 0..b {
    //     ans *= a;
    //     ans %= 1000000007;
    // }
    // println!("{}", ans);

    // 繰り返し二乗法
    let mut ex = vec![(1, a)];
    while let Some(&(i, prev)) = ex.last() {
        if i * 2 > b {
            break;
        }
        let mut next = prev * prev;
        next %= 1000000007;
        ex.push((i * 2, next));
    }

    let mut ans = 1;
    let mut b = b;
    while b > 0 {
        let &(i, prev) = ex.last().unwrap();
        if i <= b {
            b -= i;
            ans *= prev;
            ans %= 1000000007;
        }
        ex.pop();
    }

    println!("{}", ans);
}
