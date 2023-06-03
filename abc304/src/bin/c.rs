use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn dist(p1: Point, p2: Point, d: i32) -> bool {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) <= d.pow(2)
}

fn main() {
    input! {
        n: usize,
        d: i32,
        xy: [(i32, i32); n],
    }

    let points: Vec<Point> = xy.into_iter().map(|(x, y)| Point { x, y }).collect();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            if dist(points[i], points[j], d) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut infected: Vec<bool> = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    infected[0] = true;

    while let Some(i) = queue.pop_front() {
        for &j in graph[i].iter() {
            if !infected[j] {
                infected[j] = true;
                queue.push_back(j);
            }
        }
    }

    for i in 0..n {
        if infected[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
