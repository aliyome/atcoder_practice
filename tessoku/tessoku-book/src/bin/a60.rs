use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n]
    }

    // 1-indexed
    a.insert(0, 0);

    let mut stack = vec![];
    for i in 1..=n {
        while let Some((ad, d)) = stack.pop() {
            if ad <= a[i] {
                continue;
            } else {
                print!("{} ", d);
                // もとに戻す
                stack.push((ad, d));
                break;
            }
        }
        if stack.is_empty() {
            print!("-1 ");
        }
        stack.push((a[i], i));
    }
}
