use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        bus_data: [(u64, u64); n - 1],
        q: usize,
        queries: [u64; q],
    }

    for query in queries {
        // Takahashi's initial time
        let mut cur_time = query;

        // Walk to the first bus stop
        cur_time += x;

        // Process each bus stop
        for &(p, t) in &bus_data {
            // Wait for the bus
            let wait_time = if cur_time % p == 0 {
                0
            } else {
                p - cur_time % p
            };
            cur_time += wait_time + t;
        }

        // Walk to Aoki's house
        cur_time += y;

        println!("{}", cur_time);
    }
}
