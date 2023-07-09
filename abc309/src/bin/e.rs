use proconio::input;

fn main() {
    input! {
        n: usize, // <= 3 x 10^5
        m: usize, // <= 3 x 10^5
        mut p: [usize; n-1],
        xy: [(usize, usize); m]
    };

    // p[i] := iの親
    p.insert(0, 0);
    p.insert(0, 0);

    // c[i] := iの子リスト
    let mut c = vec![vec![]; n + 1];
    for i in 1..=n {
        c[p[i]].push(i);
    }

    // O(MN) -> TLE
    // x := 加入者
    // y := 補償対象の世代数
    let mut ans = vec![false; n + 1];
    for &(x, y) in &xy {
        ans[x] = true;

        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            ans[x] = true;
            for &i in &c[x] {
                if y == 0 {
                    break;
                }
                stack.push((i, y - 1));
            }
        }
    }

    println!("{}", ans.iter().filter(|&&x| x).count());
}
