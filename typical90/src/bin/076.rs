use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    // 全体の和が10で割り切れない場合は、解は存在しない
    let total = a.iter().sum::<usize>();
    if total % 10 != 0 {
        println!("No");
        return;
    }

    // 全体の10分の1が目標値
    let target = total / 10;

    // 円形を考慮するために2周分の配列を用意する
    let mut a_twice = vec![];
    a_twice.extend_from_slice(&a);
    a_twice.extend_from_slice(&a);

    // 累積和
    let mut acc = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        acc[i + 1] = acc[i] + a_twice[i];
    }

    // lを固定して二分探索
    for l in 1..=n {
        let mut ok = l;
        let mut ng = l + n + 1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if acc[mid] - acc[l - 1] <= target {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if acc[ok] - acc[l - 1] == target {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
