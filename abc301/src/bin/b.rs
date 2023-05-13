use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut ans = vec![];
    ans.push(a[0]);
    for i in 1..n {
        let last = *ans.last().unwrap();
        if last < a[i] {
            for j in last + 1..=a[i] {
                ans.push(j);
            }
        } else if last > a[i] {
            for j in (a[i]..=last - 1).rev() {
                ans.push(j);
            }
        }
    }

    let joined = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", joined);
}
