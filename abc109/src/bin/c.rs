use proconio::input;

fn main() {
    input! {
        n: usize,
        s: isize,
        mut x: [isize; n]
    };

    if n == 1 {
        println!("{}", (x[0] - s).abs());
        return;
    }

    x.sort();

    // 各都市間の距離
    let mut dists = vec![];
    if s < x[0] {
        dists.push(x[0] - s);
    }
    for i in 0..n - 1 {
        if x[i] < s && s < x[i + 1] {
            dists.push(s - x[i]);
            dists.push(x[i + 1] - s);
        } else {
            dists.push(x[i + 1] - x[i]);
        }
    }
    if x[n - 1] < s {
        dists.push(s - x[n - 1]);
    }

    // 各都市に止まるための歩幅は最大公約数
    let mut g = dists[0] as usize;
    for i in 1..dists.len() {
        g = gcd(g, dists[i] as usize);
    }

    println!("{}", g);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
