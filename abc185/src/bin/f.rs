use proconio::input;

struct SegTree {
    size: usize,
    nodes: Vec<usize>,
}

impl SegTree {
    fn new(n: usize, a: Vec<usize>) -> Self {
        let size = n.next_power_of_two();
        let mut nodes = vec![0; 2 * size - 1];
        nodes[size - 1..size - 1 + n].clone_from_slice(&a);
        for i in (0..size - 1).rev() {
            nodes[i] = nodes[2 * i + 1] ^ nodes[2 * i + 2];
        }
        Self { size, nodes }
    }

    fn update(&mut self, i: usize, x: usize) {
        let mut i = i + self.size - 1;
        self.nodes[i] = x;
        while i > 0 {
            i = (i - 1) / 2;
            self.nodes[i] = self.nodes[2 * i + 1] ^ self.nodes[2 * i + 2];
        }
    }

    fn query(&self, a: usize, b: usize) -> usize {
        self.execute_query(a, b, 0, 0, self.size)
    }

    fn execute_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.nodes[k];
        }
        let mid = (l + r) / 2;
        let vl = self.execute_query(a, b, 2 * k + 1, l, mid);
        let vr = self.execute_query(a, b, 2 * k + 2, mid, r);
        vl ^ vr
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(usize, usize, usize); q],
    };

    let mut seg_tree = SegTree::new(n, a);

    for (t, x, y) in txy {
        if t == 1 {
            let val = seg_tree.query(x - 1, x) ^ y;
            seg_tree.update(x - 1, val);
        } else {
            println!("{}", seg_tree.query(x - 1, y));
        }
    }
}
