use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut ans = vec![];
    for i in 1..=9 {
        ans.push(i);
        dfs(&mut vec![i], &mut ans);
    }
    ans.sort();
    println!("{:?}", ans[k - 1]);
}

fn dfs(curr: &mut Vec<usize>, ans: &mut Vec<usize>) {
    let last = *curr.last().unwrap();
    if last == 0 {
        return;
    }

    for i in (0..=last - 1).rev() {
        curr.push(i);
        ans.push(
            curr.iter()
                .map(|&x| x.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
        );
        dfs(curr, ans);
        curr.pop();
    }
}
