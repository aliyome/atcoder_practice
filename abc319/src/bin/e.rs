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

    // Process the queries
    for &query in &queries {
        let mut dp = vec![0u64; n];
        dp[0] = x + query; // Start time to reach first bus stop

        // Compute the minimum time required to reach each bus stop
        for i in 1..n {
            let (p, t) = bus_data[i - 1];
            // Calculate wait time
            let wait_time = if dp[i - 1] % p == 0 {
                0
            } else {
                p - dp[i - 1] % p
            };
            dp[i] = dp[i - 1] + wait_time + t;
        }

        let final_time = dp[n - 1] + y; // Add time required to reach Aoki's house
        println!("{}", final_time);
    }
}
