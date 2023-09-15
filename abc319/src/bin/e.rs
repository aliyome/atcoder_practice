use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        bus_data: [(usize, usize); n - 1],
        q: usize,
        queries: [usize; q],
    }

    let m = 840;
    let mut time = vec![0; m];
    for i in 0..m {
        let mut curr = i + x;
        for j in 0..n - 1 {
            let (p, t) = bus_data[j];
            for k in curr.. {
                if k % p == 0 {
                    curr = k + t;
                    break;
                }
            }
        }
        time[i] = curr + y;
        time[i] -= i;
    }

    for &query in &queries {
        println!("{}", query + time[query % m]);
    }
}
