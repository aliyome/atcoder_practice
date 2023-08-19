use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n < 357 {
        println!("0");
        return;
    }

    let mut list = vec![];
    let mut rest = HashSet::new();
    rest.insert(7);
    rest.insert(5);
    rest.insert(3);
    dfs(n, 0, rest, &mut list);

    println!("{}", list.len());
}

fn dfs(n: usize, curr: usize, rest: HashSet<usize>, list: &mut Vec<usize>) {
    if curr > n {
        return;
    }

    if curr != 0 && rest.is_empty() {
        list.push(curr);
    }

    let mut r1 = rest.clone();
    r1.remove(&3);
    dfs(n, curr * 10 + 3, r1, list);

    let mut r2 = rest.clone();
    r2.remove(&5);
    dfs(n, curr * 10 + 5, r2, list);

    let mut r3 = rest.clone();
    r3.remove(&7);
    dfs(n, curr * 10 + 7, r3, list);
}
