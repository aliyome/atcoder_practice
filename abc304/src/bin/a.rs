use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, usize); n]
    };

    let mut min = std::usize::MAX;
    let mut min_i = 0;
    for i in 0..n {
        if sa[i].1 < min {
            min = sa[i].1;
            min_i = i;
        }
    }
    for i in min_i..n {
        println!("{}", sa[i].0);
    }
    for i in 0..min_i {
        println!("{}", sa[i].0);
    }
}
