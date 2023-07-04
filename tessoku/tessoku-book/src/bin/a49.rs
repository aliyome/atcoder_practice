use proconio::input;

fn main() {
    input! {
      t: usize,
      pqr: [(usize, usize, usize); t]
    }

    let mut x = vec![0isize; 21];
    x[0] = 1;

    let calc_score = |x: &Vec<isize>| x.iter().skip(1).filter(|&x| *x == 0).count();

    let op_a = |x: &mut Vec<isize>, p: usize, q: usize, r: usize| {
        x[p] += 1;
        x[q] += 1;
        x[r] += 1;
    };
    let op_b = |x: &mut Vec<isize>, p: usize, q: usize, r: usize| {
        x[p] -= 1;
        x[q] -= 1;
        x[r] -= 1;
    };

    let mut ans = vec![];
    for &(p, q, r) in &pqr {
        let mut xa = x.clone();
        let mut xb = x.clone();
        op_a(&mut xa, p, q, r);
        op_b(&mut xb, p, q, r);
        let a_score = calc_score(&xa);
        let b_score = calc_score(&xb);
        if a_score > b_score {
            x = xa;
            ans.push('A');
        } else {
            x = xb;
            ans.push('B');
        }
    }

    for c in ans {
        println!("{}", c);
    }
}
