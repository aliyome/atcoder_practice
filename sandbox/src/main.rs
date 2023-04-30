use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let n = h.min(w);
    let mut counts = vec![0; n + 1];

    // count
    for y in 1..h {
        for x in 1..w {
            let count = count_cross_edge(&c, x, y, w, h);
            if count > 0 {
                counts[count] += 1;
            }
        }
    }

    // print
    for i in 1..=n {
        if i == n {
            println!("{}", counts[i]);
        } else {
            print!("{} ", counts[i]);
        }
    }
}

fn count_cross_edge(c: &Vec<Vec<char>>, x: usize, y: usize, w: usize, h: usize) -> usize {
    if (is_cross_center(c, x, y, w, h)) {
        return 1;
    } else {
        return 0;
    }
}

fn is_cross_center(c: &Vec<Vec<char>>, x: usize, y: usize, w: usize, h: usize) -> bool {
    if x < 1 || y < 1 || x > w - 2 || y > h - 2 {
        return false;
    }
    c[x][y] == '#'
        && c[x - 1][y] == '.'
        && c[x + 1][y] == '.'
        && c[x][y - 1] == '.'
        && c[x][y + 1] == '.'
        && c[x - 1][y - 1] == '#'
        && c[x + 1][y - 1] == '#'
        && c[x - 1][y + 1] == '#'
        && c[x + 1][y + 1] == '#'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_cross_edge() {
        assert_eq!(true, false);
    }
}
