use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    };

    let mut max = p[n - 1];
    let mut max_i = n - 1;
    for i in (0..n - 1).rev() {
        if p[i] > p[i + 1] {
            max = p[i];
            max_i = i;
            break;
        }
    }

    let mut c = 0;
    for i in max_i + 1..n {
        if p[i] < p[max_i] {
            c = c.max(p[i]);
        }
    }

    let mut front = vec![];
    front.extend(p[..max_i].iter());
    front.push(c);

    let mut back = vec![];
    back.push(max);
    for &e in &p[max_i + 1..] {
        if e != c {
            back.push(e);
        }
    }
    back.sort();
    back.reverse();

    for &e in &front {
        print!("{} ", e);
    }
    for &e in &back {
        print!("{} ", e);
    }
}
