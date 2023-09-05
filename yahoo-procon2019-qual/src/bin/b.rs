use proconio::input;

fn main() {
    input! {
        ab: [(usize, usize); 3],
    };

    let mut count = vec![0; 5];
    for (a, b) in ab {
        count[a] += 1;
        count[b] += 1;
    }
    if count.iter().all(|&c| c <= 2) {
        println!("YES");
    } else {
        println!("NO");
    }
}
