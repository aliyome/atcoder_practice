use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let max = (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64).ceil() as usize;
    let mut ans = vec![];
    for i in (1..=max).rev() {
        if i > n {
            continue;
        }
        if n < i {
            break;
        }

        n -= i;
        ans.push(i);

        if n == 0 {
            break;
        }
    }

    ans.reverse();
    for a in ans {
        println!("{}", a);
    }
}
