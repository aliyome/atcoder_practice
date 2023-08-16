use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut acc_0 = vec![0isize; n + 1];
    let mut acc_1 = vec![0isize; n + 1];
    for i in 0..n {
        acc_0[i + 1] = acc_0[i].max(0) + if a[i] == 0 { 1 } else { -1 };
        acc_1[i + 1] = acc_1[i].max(0) + if a[i] == 0 { -1 } else { 1 };
    }
    // println!("{:?} {:?}", acc_0, acc_1);

    let mut minus = 0;
    let mut plus = 0;
    for i in 1..=n {
        plus = plus.max(acc_0[i]);
        minus = minus.max(acc_1[i]);
    }

    let sum = a.iter().sum::<usize>() as isize;
    let min = sum - minus;
    let max = sum + plus;
    println!("{}", max - min + 1);
}
