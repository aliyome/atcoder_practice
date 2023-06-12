use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    // // 全探索 O(N^2) TLE
    // let mut count = 0;
    // for i in 0..n {
    //     for j in i + 1..n {
    //         if (a[i] - a[j]) % 200 == 0 {
    //             count += 1;
    //         }
    //     }
    // }

    // mod の性質（分配則？）を利用する
    // counts[j] := a[i]を200で割った余りがjになる個数を数える
    let mut counts = vec![0; 200];
    for i in 0..n {
        counts[a[i] % 200] += 1;
    }

    let mut ans = 0usize;
    for i in 0..n {
        let mod200 = a[i] % 200;
        counts[mod200] -= 1;
        ans += counts[mod200];
    }

    println!("{}", ans);
}
