use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a: Vec<usize> = a.iter().map(|x| x - 1).collect();
    let mut counts = vec![0; n];
    for &e in &a {
        counts[e] += 1;
    }

    let mut total = 0;
    for i in 0..n {
        total += nc2(counts[i]);
    }
    for e in &a {
        println!("{}", total - nc2(counts[*e]) + nc2(counts[*e] - 1));
    }
}

fn nc2(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    n * (n - 1) / 2
}
