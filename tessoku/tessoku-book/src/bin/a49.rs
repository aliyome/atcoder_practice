use proconio::input;

fn main() {
    input! {
      t: usize,
      pqr: [(usize, usize, usize); t]
    }

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

    // ビームサーチ
    #[derive(Debug, Clone)]
    struct Beam {
        score: usize,
        x: Vec<isize>,
        ans: Vec<char>,
    }
    let beam_width = 10000;
    let mut beam = vec![
        vec![
            Beam {
                x: vec![0isize; 21],
                score: 0,
                ans: vec![]
            };
            beam_width
        ];
        t + 1
    ];
    // x[0]は使わない (1-indexed)
    for i in 0..=t {
        for j in 0..beam_width {
            beam[i][j].x[0] = 1;
        }
    }

    for i in 0..t {
        let (p, q, r) = pqr[i];
        let mut candidates = vec![];
        for j in 0..beam_width {
            let b = &beam[i][j];
            let mut xa = b.x.clone();
            let mut xb = b.x.clone();
            op_a(&mut xa, p, q, r);
            op_b(&mut xb, p, q, r);
            let a_score = calc_score(&xa);
            let b_score = calc_score(&xb);
            let mut ans_a = b.ans.clone();
            let mut ans_b = b.ans.clone();
            ans_a.push('A');
            ans_b.push('B');
            candidates.push(Beam {
                x: xa,
                score: a_score,
                ans: ans_a,
            });
            candidates.push(Beam {
                x: xb,
                score: b_score,
                ans: ans_b,
            });
        }
        candidates.sort_by_key(|b| b.score);
        for (j, b) in candidates.iter().rev().take(beam_width).enumerate() {
            beam[i + 1][j] = b.clone();
        }
    }

    let mut max_j = 0;
    for j in 0..beam_width {
        if beam[t][j].score > beam[t][max_j].score {
            max_j = j;
        }
    }
    for i in 0..t {
        println!("{}", beam[t][max_j].ans[i]);
    }
}
