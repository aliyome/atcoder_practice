use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [usize; m],
    }

    // 候補者ごとの票数
    let mut map: HashMap<usize, usize> = HashMap::new();
    // 現在の順位 [(票数, 候補者)]
    let mut set: BTreeSet<(Reverse<usize>, usize)> = BTreeSet::new();

    for a in a {
        *map.entry(a).or_insert(0) += 1;
        let current = *map.get(&a).unwrap();
        if current == 1 {
            set.insert((Reverse(current), a));
        } else {
            set.remove(&(Reverse(current - 1), a));
            set.insert((Reverse(current), a));
        }
        println!("{}", set.iter().next().unwrap().1);
    }
}
