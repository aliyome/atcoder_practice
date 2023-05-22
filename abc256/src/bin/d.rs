use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut table = vec![0; 200_001];
    for (l, r) in lr {
        table[l] = table[l].max(r);
    }
    // println!("{:?}", table[0..51].iter());

    let mut start = 200_002;
    let mut end = 0;
    for i in 1..200_001 {
        if table[i] != 0 {
            if end < i {
                if start != 200_002 {
                    println!("{} {}", start, end);
                }
                start = i;
            }
            end = end.max(table[i]);
        }
    }
    println!("{} {}", start, end);
}
