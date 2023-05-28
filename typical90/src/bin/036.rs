use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
    }

    let mut uv = vec![];
    let mut min_u = 10i64.pow(9);
    let mut min_v = 10i64.pow(9);
    let mut max_u = -10i64.pow(9);
    let mut max_v = -10i64.pow(9);
    for i in 0..n {
        uv.push((xy[i].0 + xy[i].1, xy[i].0 - xy[i].1));
        min_u = min_u.min(xy[i].0 + xy[i].1);
        min_v = min_v.min(xy[i].0 - xy[i].1);
        max_u = max_u.max(xy[i].0 + xy[i].1);
        max_v = max_v.max(xy[i].0 - xy[i].1);
    }

    for _ in 0..q {
        input! {
            query: usize
        }
        let (u, v) = uv[query - 1];
        let u_dist = (u - min_u).abs().max((u - max_u).abs());
        let v_dist = (v - min_v).abs().max((v - max_v).abs());
        println!("{}", u_dist.max(v_dist));
    }
}
