use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        a: [[usize; 9]; 9],
    }

    // 1. row
    let row_ok = || {
        let mut ok = true;
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                if set.contains(&a[i][j]) {
                    ok = false;
                    break;
                }
                set.insert(a[i][j]);
            }
        }
        ok
    };

    // 2. col
    let col_ok = || {
        let mut ok = true;
        for j in 0..9 {
            let mut set = HashSet::new();
            for i in 0..9 {
                if set.contains(&a[i][j]) {
                    ok = false;
                    break;
                }
                set.insert(a[i][j]);
            }
        }
        ok
    };

    // 3. 3x3
    let block_ok = || {
        let mut ok = true;
        for ii in 0..3 {
            for jj in 0..3 {
                let mut set = HashSet::new();
                for i in 0..3 {
                    for j in 0..3 {
                        let i = ii * 3 + i;
                        let j = jj * 3 + j;
                        if set.contains(&a[i][j]) {
                            ok = false;
                            break;
                        }
                        set.insert(a[i][j]);
                    }
                }
            }
        }
        ok
    };

    if row_ok() && col_ok() && block_ok() {
        println!("Yes");
    } else {
        println!("No");
    }
}
