use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      a: [usize; n]
    }

    // 半分に分ける
    let mut a1 = vec![];
    let mut a2 = vec![];
    for i in 0..n {
        if i < n / 2 {
            a1.push(a[i]);
        } else {
            a2.push(a[i]);
        }
    }

    // それぞれの配列が取りうる値を列挙
    let sum = |list: &Vec<usize>| -> Vec<usize> {
        let mut sum = vec![];
        for i in 0usize..1 << list.len() {
            let mut s = 0;
            for j in 0..15 {
                if i & 1 << j != 0 {
                    s += list[j];
                }
            }
            sum.push(s);
        }
        sum
    };
    let mut a1_sum = sum(&a1);
    let mut a2_sum = sum(&a2);
    a1_sum.sort();
    a2_sum.sort();

    // 二分探索
    for &a1 in &a1_sum {
        let remainder = k - a1;
        if a2_sum.binary_search(&remainder).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
