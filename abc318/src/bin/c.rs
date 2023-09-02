use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        fares: [usize; n]
    }

    let mut fares = fares;
    fares.sort_by(|a, b| b.cmp(a));

    let mut total_cost = 0;
    let mut i = 0;

    while i < n {
        let mut sum = 0;
        for j in i..i + d {
            if j < n {
                sum += fares[j];
            }
        }
        if sum > p {
            total_cost += p;
            i += d;
        } else {
            total_cost += fares[i];
            i += 1;
        }
    }

    println!("{}", total_cost);
}
