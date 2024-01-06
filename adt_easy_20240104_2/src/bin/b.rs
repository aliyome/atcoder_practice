use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut edges = vec![vec![false; 100]; 100];
    edges[1][2] = true;
    edges[1][3] = true;
    edges[2][4] = true;
    edges[2][5] = true;
    edges[3][6] = true;
    edges[3][7] = true;
    edges[4][8] = true;
    edges[4][9] = true;
    edges[5][10] = true;
    edges[5][11] = true;
    edges[6][12] = true;
    edges[6][13] = true;
    edges[7][14] = true;
    edges[7][15] = true;

    if edges[a][b] {
        println!("Yes");
    } else {
        println!("No");
    }
}
