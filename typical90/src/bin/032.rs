use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut bad = vec![vec![]; n + 1];
    for (x, y) in xy {
        bad[x].push(y);
        bad[y].push(x);
    }

    // 全通り試しても10!通り = 3628800通りなので十分間に合う
    let mut list = vec![];
    dfs(
        0,
        n,
        1,
        n,
        &mut vec![0; n],
        &mut vec![false; n + 1],
        &mut list,
    );

    let calc_time = |order: &Vec<usize>| {
        let mut time = 0;
        for i in 1..=n {
            time += a[order[i - 1] - 1][i - 1];
        }
        time
    };

    let mut ans = std::usize::MAX;
    for order in list {
        let mut ok = true;
        for i in 1..n {
            if bad[order[i - 1]].contains(&order[i]) {
                ok = false;
                break;
            }
        }
        if ok {
            ans = ans.min(calc_time(&order));
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(
    depth: usize,
    size: usize,
    min: usize,
    max: usize,
    permutation: &mut Vec<usize>,
    used: &mut Vec<bool>,
    list: &mut Vec<Vec<usize>>,
) {
    if depth == size {
        list.push(permutation.clone());
        return;
    }

    for i in min..=max {
        if !used[i] {
            permutation[depth] = i;
            used[i] = true;
            dfs(depth + 1, size, min, max, permutation, used, list);
            used[i] = false;
        }
    }
}
