use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut max_i = 0;
    let mut max = 0;
    for i in 1..=n {
        if graph[i].len() > max {
            max = graph[i].len();
            max_i = i;
        }
    }

    println!("{}", max_i);
}
