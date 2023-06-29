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

        let v = [
            (0isize, 1isize),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for &(di, dj) in v.iter() {
            let mut str = vec![];
            for _ in 0..n {
                let mut jj = j as isize + dj;
                let mut ii = i as isize + di;
                if jj < 0 {
                    jj += n as isize;
                }
                if ii < 0 {
                    ii += n as isize;
                }
                j = jj as usize % n;
                i = ii as usize % n;
                str.push(a[i][j]);
            }
            list.push(str.iter().collect::<String>());
        }
        list.sort();
        list.reverse();
        list[0].clone()
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
