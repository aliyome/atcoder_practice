use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n],
        mut b: [isize; m],
    };

    a.sort();
    b.sort();

    let mut ans = std::isize::MAX;
    for a in a {
        let i = b.lower_bound(a);
        if i < m {
            ans = ans.min((a - b[i]).abs());
        }
        if i >= 1 {
            ans = ans.min((a - b[i - 1]).abs());
        }
    }

    println!("{}", ans);
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
