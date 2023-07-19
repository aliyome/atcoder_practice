use std::{collections::HashSet, vec};

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    // 相性の悪いペアをSetで管理する
    let mut bad_pairs = vec![];
    for &(a, b) in &ab {
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        bad_pairs.push(set);
    }

    // 全チーム分けをDFSで列挙する
    let ans = dfs(n, t, m, 1, &bad_pairs, &mut vec![]);
    println!("{}", ans)
}

fn dfs(
    n: usize,
    t: usize,
    m: usize,
    member: usize,
    bad_pairs: &Vec<HashSet<usize>>,
    teams: &mut Vec<Vec<usize>>,
) -> usize {
    // 全チーム分けができたら終了
    if member > n {
        return if teams.len() == t { 1 } else { 0 };
    }

    let mut res = 0;

    // 新しいチームを作る場合
    teams.push(vec![member]);
    res += dfs(n, t, m, member + 1, bad_pairs, teams);
    teams.pop();

    // 既存チームに参加する場合
    for i in 0..teams.len() {
        if can_add(member, &teams[i], bad_pairs) {
            teams[i].push(member);
            res += dfs(n, t, m, member + 1, bad_pairs, teams);
            teams[i].pop();
        }
    }

    res
}

fn can_add(new_member: usize, team: &Vec<usize>, bad_pairs: &Vec<HashSet<usize>>) -> bool {
    let mut team_set = team.iter().map(|&x| x).collect::<HashSet<_>>();
    team_set.insert(new_member);
    for bad in bad_pairs {
        if bad.intersection(&team_set).eq(bad) {
            return false;
        }
    }
    true
}
