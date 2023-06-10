use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    // O(N)
    let mut b: Vec<(usize, usize)> = a.clone().into_iter().enumerate().collect();
    // O(NlogN)
    b.sort_by(|x, y| x.1.cmp(&y.1));
    // O(N)
    let mut c = vec![];
    let mut prev = 0;
    for i in 0..n {
        let (index, val) = b[i];
        if i == 0 {
            c.push((index, 1));
            prev = 1;
            continue;
        }
        if val == b[i - 1].1 {
            c.push((index, prev));
        } else {
            c.push((index, prev + 1));
            prev += 1;
        }
    }

    // O(NlogN)
    c.sort_by(|x, y| x.0.cmp(&y.0));

    // O(N)
    println!(
        "{}",
        c.iter()
            .map(|x| x.1.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
