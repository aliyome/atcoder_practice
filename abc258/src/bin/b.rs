use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    let mut list = vec![];

    let walk = |i: usize, j: usize| -> String {
        let mut list = vec![];
        let mut i = i;
        let mut j = j;
        for _ in 0..n {
            j = (j + 1) % n;
            i = (i + 1) % n;
            list.push(a[i][j]);
        }
        list.iter().collect::<String>()
    };

    for i in 0..n {
        for j in 0..n {
            list.push(walk(i, j));
        }
    }

    list.sort();
    list.reverse();

    println!("{}", list[0]);
}
