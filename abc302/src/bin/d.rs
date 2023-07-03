use proconio::input;

fn main() {
    input! {
        n: usize, // 2x10^5
        m: usize, // 2x10^5
        d: i64, // 10^18
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();

    let mut ans = -1;
    for &a in &a {
        let i = b.upper_bound(a + d);
        if i >= 1 {
            if (a - b[i - 1]).abs() <= d {
                ans = ans.max(a + b[i - 1]);
            }
        }
        if i < m {
            if (a - b[i]).abs() <= d {
                ans = ans.max(a + b[i]);
            }
        }
        if i + 1 < m {
            if (a - b[i + 1]).abs() <= d {
                ans = ans.max(a + b[i + 1]);
            }
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

// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         d: i64,
//         mut a: [i64; n],
//         mut b: [i64; m],
//     }

//     a.sort();
//     b.sort();

//     let mut ans = -1;
//     for &a_val in &a {
//         let idx = match b.binary_search(&(a_val + d + 1)) {
//             Ok(i) => i,
//             Err(i) => i,
//         };
//         if idx > 0 {
//             let b_val = b[idx - 1];
//             if (a_val - b_val).abs() <= d {
//                 ans = ans.max(a_val + b_val);
//             }
//         }
//     }

//     println!("{}", ans);
// }
