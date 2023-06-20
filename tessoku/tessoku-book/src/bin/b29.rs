use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    let mut ex = vec![(1, a)];
    while let Some(&(i, prev)) = ex.last() {
        if i * 2 > b {
            break;
        }
        let mut next = prev * prev;
        next %= MOD;
        ex.push((i * 2, next));
    }

    let mut b = b;
    let mut ans = 1;
    while b > 0 {
        let &(i, prev) = ex.last().unwrap();
        if i <= b {
            b -= i;
            ans *= prev;
            ans %= MOD;
        }
        ex.pop();
    }

    println!("{}", ans);
}
