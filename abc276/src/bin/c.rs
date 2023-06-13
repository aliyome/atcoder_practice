use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [isize; n]
    };

    p.insert(0, -1);
    p.push(10isize.pow(9));

    for i in (0..n).rev() {
        if p[i] < p[i + 1] {
            continue;
        }
        let mut x = 0;
        let mut pp = p[i..p.len() - 1].to_vec();
        pp.sort();
        pp.reverse();
        for &pp in pp.iter() {
            if pp < p[i] {
                x = pp;
                break;
            }
        }
        let mut q = p[..i].to_vec();
        q.push(x);

        for j in 0..pp.len() {
            if pp[j] == x {
                continue;
            }
            q.push(pp[j]);
        }
        println!(
            "{}",
            q[1..q.len()]
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    }
}
