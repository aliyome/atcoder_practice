use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.insert(0, 0);

    let mut from = vec![vec![]; n + 1];
    for i in 1..=n {
        from[a[i]].push(i);
    }

    let mut cnt = std::usize::MAX;
    let mut used = vec![false; n + 1];
    used[2] = true;
    dfs(2, &from, 0, &mut used, &mut cnt);

    if cnt == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", cnt);
    }
}

fn dfs(pos: usize, from: &Vec<Vec<usize>>, depth: usize, used: &mut Vec<bool>, cnt: &mut usize) {
    if pos == 1 {
        *cnt = (*cnt).min(depth);
        return;
    }

    for &j in from[pos].iter() {
        if used[j] {
            continue;
        }
        used[j] = true;
        dfs(j, from, depth + 1, used, cnt);
    }
}
