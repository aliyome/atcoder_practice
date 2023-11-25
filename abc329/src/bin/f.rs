use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        queries: [(usize, usize); q],
    }

    let mut list = vec![HashSet::new(); n + 1];
    for i in 0..n {
        list[i + 1].insert(c[i]);
    }

    for (a, b) in queries {
        let mut sa = HashSet::new();
        std::mem::swap(&mut sa, &mut list[a]);

        let mut sb = HashSet::new();
        std::mem::swap(&mut sb, &mut list[b]);

        let sb = if sa.len() < sb.len() {
            sb.extend(sa);
            sb
        } else {
            sa.extend(sb);
            sa
        };

        println!("{}", sb.len());

        list[b] = sb;
    }
}
