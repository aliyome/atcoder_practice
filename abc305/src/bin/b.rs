use proconio::input;

fn main() {
    input! {
        p: char,
        q: char
    };

    let dists = vec![0, 3, 1, 4, 1, 5, 9];
    let mut sum = dists.clone();
    for i in 1..dists.len() {
        sum[i] += sum[i - 1];
    }

    let p = p as usize - 'A' as usize;
    let q = q as usize - 'A' as usize;

    if p < q {
        println!("{}", sum[q] - sum[p]);
    } else {
        println!("{}", sum[p] - sum[q]);
    }
}
