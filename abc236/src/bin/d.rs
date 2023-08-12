use proconio::input;

fn main() {
    input! {
        n: usize, // <= 8
    };

    // bit_point[bit] := 今まで選んだ人の集合がbitのときの楽しさ
    let mut bit_point = vec![vec![0; 2 * n + 1]; 2 * n + 1];
    for i in 1..2 * n {
        input! {
            a: [usize; 2*n - i]
        }
        for j in 0..a.len() {
            bit_point[i][i + j + 1] = a[j];
        }
    }

    let mut used = vec![false; 2 * n + 1];
    let mut ans = vec![];
    dfs(n, 0, &bit_point, &mut used, &mut ans);

    println!("{}", *ans.iter().max().unwrap());
}

fn dfs(
    n: usize,
    point: usize,
    bit_point: &Vec<Vec<usize>>,
    used: &mut Vec<bool>,
    ans: &mut Vec<usize>,
) {
    let mut next_1 = 0;
    for i in 1..=2 * n {
        if !used[i] {
            next_1 = i;
            break;
        }
    }

    if next_1 == 0 {
        ans.push(point);
        return;
    }

    used[next_1] = true;
    for next_2 in next_1 + 1..=2 * n {
        if used[next_2] {
            continue;
        }
        used[next_2] = true;
        dfs(n, point ^ bit_point[next_1][next_2], bit_point, used, ans);
        used[next_2] = false;
    }
    used[next_1] = false;
}
