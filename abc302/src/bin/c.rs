use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }

    // 全探索
    let perm = s.iter().permutations(n).collect_vec();
    for p in perm {
        let mut ok = true;
        for i in 0..p.len() - 1 {
            let mut count = 0;
            for j in 0..m {
                if p[i].chars().nth(j) != p[i + 1].chars().nth(j) {
                    count += 1;
                }
            }
            if count > 1 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
