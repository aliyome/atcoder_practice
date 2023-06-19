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

    // s の最寄りの要素を探す
    let r = x.lower_bound(s);
    let l = r - 1;

    // 各都市間の距離
    let mut dists = vec![];
    for i in 0..l {
        dists.push(x[i + 1] - x[i]);
    }
    dists.push(s - x[l]);
    dists.push(x[r] - s);
    for i in r..n - 1 {
        dists.push(x[i + 1] - x[i]);
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
