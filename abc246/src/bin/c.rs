use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: isize,
        x: isize,
        mut a: [isize; n]
    };

    for i in 0..n {
        let cnt = a[i] / x;
        if cnt <= k {
            a[i] -= cnt * x;
            k -= cnt;
        } else {
            a[i] -= k * x;
            k = 0;
        }
        if k == 0 {
            break;
        }
    }

    if k > 0 {
        a.sort();
        a.reverse();
        for i in 0..n {
            if a[i] > 0 {
                a[i] = 0;
                k -= 1;
            }
            if k == 0 {
                break;
            }
        }
    }

    println!("{}", a.iter().sum::<isize>());
}
