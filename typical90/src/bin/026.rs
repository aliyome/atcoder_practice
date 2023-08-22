use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut edge = vec![vec![]; n];
    for &(a, b) in &ab {
        edge[a - 1].push(b - 1);
        edge[b - 1].push(a - 1);
    }

    let mut color = vec![0; n];
    color[0] = 1;
    dfs(&edge, 0, &mut color, 1);

    let c1 = color.iter().filter(|&&c| c == 1).count();
    if c1 >= n / 2 {
        color
            .iter()
            .enumerate()
            .filter(|&(i, &c)| c == 1)
            .take(n / 2)
            .for_each(|(i, _)| print!("{} ", i + 1));
    } else {
        color
            .iter()
            .enumerate()
            .filter(|&(i, &c)| c == 2)
            .take(n / 2)
            .for_each(|(i, _)| print!("{} ", i + 1));
    }
}

fn dfs(edge: &Vec<Vec<usize>>, v: usize, color: &mut Vec<usize>, prev: usize) {
    let next = if prev == 1 { 2 } else { 1 };
    for &u in &edge[v] {
        if color[u] != 0 {
            continue;
        }
        color[u] = next;
        dfs(edge, u, color, next);
    }
}
