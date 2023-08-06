use proconio::input;

fn main() {
    input! {
        n: usize
    };

    // DFS で全探索
    let mut ans = vec![];
    dfs(n, 1, &mut vec!['('], &mut ans);

    ans.sort();
    for s in ans.iter() {
        println!("{}", s);
    }
}

fn dfs(n: usize, open: usize, s: &mut Vec<char>, ans: &mut Vec<String>) {
    // 終端条件
    if s.len() == n {
        if open == 0 {
            ans.push(s.iter().collect());
        }
        return;
    }

    // ( を追加
    s.push('(');
    dfs(n, open + 1, s, ans);
    s.pop();

    // ) を追加
    if open > 0 {
        s.push(')');
        dfs(n, open - 1, s, ans);
        s.pop();
    }
}
