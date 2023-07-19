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
    println!("{}", dfs(n, t, m, 1, &bad_pairs, &mut vec![]));
}

// 既存のチームに新しいメンバーを加えても大丈夫かどうかを判定する
fn can_add(new_member: usize, team: &Vec<usize>, bad_pairs: &Vec<HashSet<usize>>) -> bool {
    let mut team = team.iter().copied().collect::<HashSet<_>>();
    team.insert(new_member);
    for bad in bad_pairs.iter() {
        if bad.is_subset(&team) {
            return false;
        }
    }
    true
}

fn dfs(
    n: usize,
    t: usize,
    m: usize,
    member: usize,
    bad_pairs: &Vec<HashSet<usize>>,
    teams: &mut Vec<Vec<usize>>,
) -> usize {
    // 全メンバーがいずれかのチームに入ったら終了
    if member > n {
        // 規定のチーム数にならなかったら除外
        if teams.len() == t {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;

    // 既存のチームに加入する
    for i in 0..teams.len() {
        if can_add(member, &teams[i], bad_pairs) {
            teams[i].push(member);
            res += dfs(n, t, m, member + 1, bad_pairs, teams);
            teams[i].pop();
        }
    }

    // 新規チームに加入する
    teams.push(vec![member]);
    res += dfs(n, t, m, member + 1, bad_pairs, teams);
    teams.pop();

    res
}
