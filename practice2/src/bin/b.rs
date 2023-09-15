use ac_library::{Additive, FenwickTree, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        q:usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    };

    let mut tree = FenwickTree::new(n, 0usize);
    for i in 0..n {
        tree.add(i, a[i]);
    }
    for &(t, lp, rx) in &queries {
        if t == 0 {
            tree.add(lp, rx);
        } else {
            println!("{}", tree.sum(lp..rx));
        }
    }

    // Fenwick ではなく Segtree を使う場合
    // let mut tree = Segtree::<Additive<usize>>::new(n);
    // for i in 0..n {
    //     tree.set(i, a[i]);
    // }
    // for &(t, lp, rx) in &queries {
    //     if t == 0 {
    //         tree.set(lp, tree.get(lp) + rx);
    //     } else {
    //         println!("{}", tree.prod(lp..rx));
    //     }
    // }
}
