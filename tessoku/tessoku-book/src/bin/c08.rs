use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      st: [(Chars, usize); n]
    }

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (s, t) in &st {
        if t == &1 {
            println!("{}", s.iter().collect::<String>());
            return;
        }
    }

    let mut ans = vec![];
    for &d1 in &digits {
        for &d2 in &digits {
            for &d3 in &digits {
                for &d4 in &digits {
                    let mut ok = true;
                    for (s, t) in st.iter() {
                        let mut cnt = 0;
                        cnt += if s[0] == d1 { 1 } else { 0 };
                        cnt += if s[1] == d2 { 1 } else { 0 };
                        cnt += if s[2] == d3 { 1 } else { 0 };
                        cnt += if s[3] == d4 { 1 } else { 0 };
                        if (*t == 2 && cnt == 3) || (*t == 3 && cnt <= 2) {
                            continue;
                        } else {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        ans.push(format!("{}{}{}{}", d1, d2, d3, d4));
                    }
                }
            }
        }
    }

    if ans.len() == 1 {
        println!("{}", ans[0]);
    } else {
        println!("Can't Solve");
    }
}
