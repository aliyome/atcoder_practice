use proconio::input;

fn main() {
    input! {
        n: usize, // <= 8
    };

    // a[i][j] := i と j の相性スコア
    // i と j は 1-indexed
    let mut a = vec![vec![0; 2 * n + 1]; 2 * n + 1];
    for i in 1..2 * n {
        input! {
            aa: [usize; 2*n - i]
        }
        for j in 0..2 * n - i {
            a[i][i + j + 1] = aa[j];
        }
    }

    // used[i] := 人 i の組分けが決まったかどうか
    let mut used = vec![false; 2 * n + 1];
    used[0] = true;
    let mut ans = vec![];
    dfs(&a, &mut vec![], &mut used, n, &mut ans);

    println!("{}", ans.iter().max().unwrap());
}

// 相性の計算
fn calc(list: &Vec<(usize, usize)>, a: &Vec<Vec<usize>>) -> usize {
    let mut ans = 0;
    for &(i, j) in list {
        ans ^= a[i][j];
    }
    ans
}

fn dfs(
    a: &Vec<Vec<usize>>,
    list: &mut Vec<(usize, usize)>,
    used: &mut Vec<bool>,
    n: usize,
    ans: &mut Vec<usize>,
) {
    if list.len() == n {
        ans.push(calc(list, a));
    }

    // まだ使われていない最小の人を探す
    let mut first = 0;
    for i in 1..=2 * n {
        if !used[i] {
            first = i;
            break;
        }
    }
    used[first] = true;

    // その人とまだ使われていない人の組み合わせを全探索する
    for second in first + 1..=2 * n {
        if !used[second] {
            used[second] = true;
            list.push((first, second));
            dfs(a, list, used, n, ans);
            list.pop();
            used[second] = false;
        }
    }
    used[first] = false;
}
