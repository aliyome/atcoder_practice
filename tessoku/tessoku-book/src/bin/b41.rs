use proconio::input;

fn main() {
    input! {
      x: usize,
      y: usize,
    }

    let mut ans = vec![];
    ans.push((x, y));

    while let Some(&(x, y)) = ans.last() {
        if (x, y) == (1, 1) {
            break;
        }
        if x > y {
            ans.push((x - y, y));
        } else {
            ans.push((x, y - x));
        }
    }

    println!("{}", ans.len() - 1);
    for &(x, y) in ans.iter().rev().skip(1) {
        println!("{} {}", x, y);
    }
}
