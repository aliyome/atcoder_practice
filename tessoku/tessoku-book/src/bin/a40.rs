use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut map = HashMap::new();
    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    let ans = map
        .values()
        .filter(|&x| *x >= 3)
        .fold(0, |acc, x| acc + combination(*x, 3));

    println!("{}", ans);
}

// n!
pub fn factorial(n: usize) -> usize {
    (1..=n).fold(1, |x, y| x * y)
}

// nPr (https://ja.wikipedia.org/wiki/%E9%A0%86%E5%88%97#%E9%A0%86%E5%88%97%E3%81%AE%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92)
pub fn permutation(n: usize, r: usize) -> usize {
    if n < r {
        0
    } else {
        (n - r + 1..=n).fold(1, |x, y| x * y)
    }
}

// nCr (https://ja.wikipedia.org/wiki/%E7%B5%84%E5%90%88%E3%81%9B_(%E6%95%B0%E5%AD%A6))
pub fn combination(n: usize, r: usize) -> usize {
    if n < r {
        0
    } else {
        permutation(n, r) / factorial(r)
    }
}
