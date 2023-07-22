use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut visited = vec![false; n];
    let mut idx = 0;
    let mut path = vec![];

    while !visited[idx] {
        visited[idx] = true;
        path.push(idx);
        idx = a[idx];
    }

    let cycle_start = path.iter().position(|&x| x == idx).unwrap();
    let cycle = &path[cycle_start..];

    println!("{}", cycle.len());
    for &node in cycle {
        print!("{} ", node + 1);
    }
}
