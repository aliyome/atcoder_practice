use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut next = vec![0; n + 1];
    let mut back = vec![0; n + 1];

    for i in 1..=n {
        next[i] = i;
        back[i] = i;
    }

    for _ in 0..q {
        input! {
            c: usize
        };

        match c {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                };
                next[x] = y;
                back[y] = x;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                };
                next[x] = x;
                back[y] = y;
            }
            3 => {
                input! {
                    mut x: usize,
                };
                // 先頭にたどる
                while x != back[x] {
                    x = back[x];
                }
                // 後ろにたどる
                let mut list = vec![];
                list.push(x);
                while x != next[x] {
                    x = next[x];
                    list.push(x);
                }
                // 出力
                let str = list
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{} {}", list.len(), str);
            }
            _ => unreachable!(),
        }
    }
}
