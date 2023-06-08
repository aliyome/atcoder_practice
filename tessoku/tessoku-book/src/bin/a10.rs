use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n],
      d: usize,
      lr: [(usize, usize); d],
    }

    let mut lefts = a.clone();
    let mut rights = a.clone();
    rights.reverse();
    for i in 1..n {
        lefts[i] = lefts[i].max(lefts[i - 1]);
        rights[i] = rights[i].max(rights[i - 1]);
    }

    rights.reverse();

    for (l, r) in lr {
        let left = lefts[l - 2];
        let right = rights[r];
        println!("{}", left.max(right));
    }
}
