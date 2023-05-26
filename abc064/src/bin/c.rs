use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut counts = vec![0; 9];
    // let mut mins = vec![0; 9];
    for a in a {
        let rank = a / 400;
        if rank >= 8 {
            counts[8] += 1;
        } else {
            counts[rank] += 1;
        }
        // mins[rank] = mins[rank].min(a);
    }

    let mut min = 0;
    for i in 0..8 {
        if counts[i] > 0 {
            min += 1;
        }
    }

    let max = min + counts[8];

    if min == 0 && counts[8] > 0 {
        min = 1;
    }

    println!("{} {}", min, max);
}
