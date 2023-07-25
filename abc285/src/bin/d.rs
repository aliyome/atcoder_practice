use std::collections::{BTreeSet, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut names = HashSet::new();
    let mut rest = BTreeSet::new();

    // 最初はSが入っている
    for (s, t) in st.iter() {
        names.insert(s.clone());
        rest.insert((t.clone(), s.clone()));
    }

    // Tを入れていく
    while rest.len() > 0 {
        let mut ok = false;
        for ts in rest.iter() {
            let (t, s) = ts;
            if !names.contains(t) {
                names.insert(t.clone());
                names.remove(&s.clone());
                rest.remove(&ts.clone());
                ok = true;
                break;
            }
        }
        if !ok {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
