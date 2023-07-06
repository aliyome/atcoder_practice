use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    };

    // 1-indexed
    a.insert(0, 0);

    // dist[i] := 都市iのテレポート回数
    let mut dist = vec![0; n + 1];
    // city[i] := テレポート回数iの都市
    let mut city = vec![0; n + 1];

    // 現在地
    let mut cur = 1;
    // ループ地点(末端)
    let mut loop_end = 0;
    // ループ地点(先頭)
    let mut loop_start = 0;
    for d in 0..n {
        let to = a[cur];
        if dist[to] > 0 {
            dist[cur] = d;
            city[d] = cur;
            loop_start = to;
            loop_end = cur;
            break;
        }
        dist[cur] = d;
        city[d] = cur;
        cur = to;
    }

    if k <= dist[loop_start] {
        println!("{}", city[k]);
        return;
    }

    let loop_size = dist[loop_end] - dist[loop_start] + 1;
    let remainder = k - dist[loop_start];
    let normalized_dist = dist[loop_start] + remainder % loop_size;

    println!("{}", city[normalized_dist]);
}
