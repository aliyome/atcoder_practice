use proconio::input;

fn main() {
    input! {
        n: usize, // <= 8
        k: usize, // <=10^9
        t: [[usize; n]; n]
    };

    // 地点 0 からスタートして、すべての地点を1度訪れた時のコスト + 地点 0 に戻るコスト
    let mut ans = 0;
    let mut visit = vec![false; n];
    visit[0] = true;
    dfs(0, 0, &t, &visit, k, &mut ans);

    println!("{}", ans);
}

fn dfs(
    current: usize,
    cost: usize,
    t: &Vec<Vec<usize>>,
    visit: &Vec<bool>,
    k: usize,
    ans: &mut usize,
) {
    // すべての地点を訪れたか
    if visit.iter().all(|&v| v) {
        // 地点 0 に戻るコストを加算
        if cost + t[current][0] == k {
            *ans += 1;
        }
        return;
    }

    // すべての地点を訪れていない場合、次の地点を選ぶ
    for i in 0..t.len() {
        if visit[i] {
            continue;
        }
        let mut visit = visit.clone();
        visit[i] = true;
        dfs(i, cost + t[current][i], t, &visit, k, ans);
    }
}
