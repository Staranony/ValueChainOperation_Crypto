use sha2::{Sha256, Digest};
use std::time::Instant;
use std::io::{self, Write}; // Let's see how it works in real time

fn main() {
    println!("--- [CPU Mining Simulation: Bitcoin mining] ---");
    println!("it is not just Comparsion of numbers, it computes every heavy SHA-256 hashrate");
    println!("Realse Mode on! (cargo run --release --bin heavy_mining)\n");

    // Level 5 ("00000"): Below 1 second ~ seconds
    mine_block(5);

    // Level 6 ("000000") : Now it waits several seconds
    // add 0 makes, longer waiting time exactly 16 times more slower
    mine_block(6);

    // Warning: if you have great computer, try block 7 to estimate time (probably it takes up to several minutes)
    mine_block(7);
}

fn mine_block(difficulty: usize) {
    let target = "0".repeat(difficulty);
    println!("-------------------------------------------------------------");
    println!("Level {} Challenge! (Goal: Finding '{}' Number or String)", difficulty, target);
    println!("-> Pressing Enter make CPU mourning :(");

    // Showing Speed of CPU parameters
    let start = Instant::now();
    let mut nonce = 0u64;
    let mut last_report = Instant::now();

    loop {
        // 1. heavy loading: Generate SHA-256
        let input = format!("my_block_data:{}", nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let hash_string = hex::encode(result);

        // 2. Check the answer
        if hash_string.starts_with(&target) {
            let duration = start.elapsed();
            println!("\n\n Find it! (Nonce: {})", nonce);
            println!("Hash value: {}...", &hash_string[0..30]);
            println!("Spent time: {:.2} second", duration.as_secs_f64());
            println!("Total attempts: {} times", nonce);

            // Compute Hashrate(Instructions For Seconds)
            let hashrate = nonce as f64 / duration.as_secs_f64();
            println!(" Your CPU velocity: approximately {:.0} Hash per second(H/s)", hashrate);
            break;
        }
    
    // 3. Ongoing Dashboard
    // Not to user to think in a  way it's going wrong
        if last_report.elapsed().as_secs_f64() > 0.5 {
            print!("\r Now it's mining very hard... Attempts: {} attempt", nonce);
            io::stdout().flush().unwrap();
            last_report = Instant::now();
        }

        nonce += 1;
    }
}