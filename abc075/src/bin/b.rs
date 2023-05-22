use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h]
    };

    let v = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for y in 0..h as i64 {
        for x in 0..w as i64 {
            if s[y as usize].chars().nth(x as usize).unwrap() == '#' {
                print!("#");
                continue;
            }
            let mut cnt = 0;
            for &(vx, vy) in &v {
                let xx = x + vx;
                let yy = y + vy;
                if xx < 0 || xx >= w as i64 || yy < 0 || yy >= h as i64 {
                    continue;
                }
                if s[yy as usize].chars().nth(xx as usize).unwrap() == '#' {
                    cnt += 1;
                }
            }
            print!("{}", cnt);
        }
        println!("");
    }
}
