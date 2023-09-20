use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    };

    let mut seg_max = Segtree::<Max<isize>>::new(n + 1);
    for i in 0..n {
        seg_max.set(i + 1, a[i]);
    }

    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { x: usize, v: isize };
                seg_max.set(x, v);
            }
            2 => {
                input! { l: usize, r: usize };
                println!("{}", seg_max.prod(l..=r));
            }
            3 => {
                input! { x: usize, v: isize };
                println!("{}", seg_max.max_right(x, |&a| a < v));
            }
            _ => unreachable!(),
        }
    }
}
