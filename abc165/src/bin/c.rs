use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize, usize, isize, isize); q]
    };

    // 全探索 10!
    let mut all_list = vec![];
    let rest = (1..=m).collect::<Vec<usize>>();
    dfs(n, rest, &mut all_list, &mut vec![]);

    let mut ans = 0;
    for list in all_list.iter() {
        let mut sum = 0;
        for &(a, b, c, d) in &abcd {
            if list[b - 1] - list[a - 1] == c {
                sum += d;
            }
        }
        ans = ans.max(sum);
    }

    println!("{}", ans);
}

fn dfs(n: usize, rest: Vec<usize>, all_list: &mut Vec<Vec<isize>>, list: &mut Vec<isize>) {
    if list.len() == n {
        all_list.push(list.clone());
        return;
    }

    for &r in &rest {
        if list.last().is_some() && list.last().unwrap() > &(r as isize) {
            continue;
        }
        list.push(r as isize);
        dfs(n, rest.clone(), all_list, list);
        list.pop();
    }
}
