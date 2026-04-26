use std::time::{Duration, Instant};
use std::thread;

// --- Configuration ---
const BLOCK_SIZE: usize = 8; // Matching the paper's DES/3DES examples
const MAC_CHECK_DELAY_MS: u64 = 20; // Simulated time for MAC check(Success path)
const PADDING_ERROR_DELAY_MS: u64 = 2; // Simulated time for Padding error (Fast fail)
const NOISE_MAGNITUDE_MS: u64 = 1; // Simulated network jitter

// SPRT Thresholds (Log-likelihood)
// These correspond to the error bounds epilon+ and epsilon- in the paper [cite: 181]
const THRESHOLD_ACCEPT: f64 = 5.0;
const THRESHOLD_REJECT: f64 = -5.0;

// --- Mock Oracle (The Vulnerable Server) ---
struct MockOracle {
    key: [u8; BLOCK_SIZE],
    secret_message: Vec<u8>,
}

impl MockOracle {
    fn new(secret: &str) -> Self {
        // Pad the secret to block size using PKCS#7 (RFC 5652) logic
        let mut msg = secret.as_bytes().to_vec();
        let padding_len = BLOCK_SIZE - (msg.len() % BLOCK_SIZE);
        let padding_byte = (padding_len - 1) as u8;
        // // Clarification: TLS/PKCS7 usually uses value = len.
        // Paper says "LEN is a single byte... length of PAD... bytes equal to LEN"
        // However, standard PKCS7 for 8 byte block : if 1 byte pad, value is 0x01.
        // Paper notation Check1 [cite: 69] uses R = (i-1)...(i-1) where i is length.
        // This implies for length 1, value is 0. This matches the paper's specific notation
        // We will stick strictly to the paper's formula: Pad value = length - 1.

        msg.extend(std::iter::repeat(padding_byte).take(padding_len));
        MockOracle {
            key: [0xAA; BLOCK_SIZE], // Fixed key for simulation
            secret_message: msg,
        }
    }

    /// Simulates capturing a legitimate session
    /// Returns (y_prev, y)where y is the target block and y_prev is the IV.
    fn capture_session(&self, block_index: usize) -> (Vec<u8>, Vec<u8>) {
        /// In a real attack, we capture ciphertext off the wire.
        /// Here, we perform encryption to generate the valid ciphertext for the attacker to see.
        let chunks: Vec<&[u8]> = self.secret_message.chunks(BLOCK_SIZE).collect();
        let target_plaintext = chunks[block_index];

        // Simulate previous ciphertext block (y_prev) acting as IV
        let y_prev = vec![0x00; BLOCK_SIZE]; // Simplified IV for this block
        // Encrypt: y = Enc(plaintext XOR y_prev)
        // We use a dummy XOR-cipher for simplicity as we only need the algebraic property
        let mut y = vec![0u8; BLOCK_SIZE];
        for i in 0..BLOCK_SIZE {
            y[i] = target_plaintext[i] ^ y_prev[i] ^ self.key[i];
        }
        (y_prev, y)
    }

    /// The Oracle Function O(ciphertext) [cite:43]
    /// Receives ciphertext, decrypts, checks padding, and returns timing.
    fn query(&self, iv: &[u8], ciphertext: &[u8]) -> Duration {
        let start = Instant::now();

        // 1. Decrypt: D(y)
        let mut decrypted = vec![0u8; BLOCK_SIZE];
        for i in 0..BLOCK_SIZE {
            decrypted[i] = ciphertext[i] ^ self.key[i];
        }
        // 2. CBC Unchain: P = D(y) XOR IV
        let mut plaintext = vec![0u8; BLOCK_SIZE];
        for i in 0..BLOCK_SIZE {
            plaintext[i] = decrypted[i] ^ iv[i];
        }
        // 3. Check Padding
        /// Paper[cite:26]: "PAD is required ... to consist of l bytes equal to l"
        /// in our implementation of the paper's logic, the last byte is the length indicator 'l'
        let l = plaintext[BLOCK_SIZE - 1] as usize;
        let pad_len = l + 1; // Assuming 0-indexed byte value (0x00 means 1 byte)

        let mut padding_valid = true;
        if pad_len > BLOCK_SIZE {
            padding_valid = false;
        } else {
            // Check that the preceding bytes are alse 'l'
            for k in 0..pad_len {
                if plaintext[BLOCK_SIZE - 1 - k] != (l as u8) {
                    padding_valid = false;
                    break;
                }
            }
        }
        // 4. Side Channel Response
        let base_delay = if padding_valid {
            // Padding OK -> Proceed to MAC check (Expensive)
            MAC_CHECK_DELAY_MS
        } else {
            // Padding Error -> Abort (Cheap)
            PADDING_ERROR_DELAY_MS
        };
        // Add simulated network noise
        let noise = (start.elapsed().subsec_nanos() % (NOISE_MAGNITUDE_MS as u32 * 1_000_000)) as u64;
        let total_delay_ns = (base_delay * 1_000_000) + noise as u64;

        // Sleep to simualate the time processing
        thread::sleep(Duration::from_nanos(total_delay_ns));

        start.elapsed()
    }
}


// --- The Attacker ---

/// Generates a simple random byed for simulation
fn random_byte() -> u8 {
    (Instant::now().elapsed().subsec_nanos() % 256) as u8
}

/// THe Check4 Algorithm [cite:223]
/// Tests if a candidate suffix 'u' is correct by querying the Oracle.
fn check4(oracle: &MockOracle, u: &[u8], block_index: usize) -> bool{
    // Sequential Probability Ratio Test (SPRT) variables
    let mut log_likelihood_ratio: f64 = 0.0;

    /// Statistical Parameters for the timing distributions D_R and D_W
    /// These would be learned during a calibration phase (Section 2.2).
    let mu_r = MAC_CHECK_DELAY_MS as f64; // Mean time for valid padding (MAC error)
    let mu_w = PADDING_ERROR_DELAY_MS as f64; // Mean time for invalid padding
    let sigma: f64 = 0.5; // Assumed standard deviation
    let mut loop_count = 0;

    // We loop until the sequential text decideds [cite: 242]
    loop {
            loop_count += 1;
            // Try 100 times if not terminates
            if loop_count > 100 {
                return false;
            }
        // 1. Wait for a new session and get current y and y' [cite: 227]
            let (y_prime, y) = oracle.capture_session(block_index);
            let i = u.len();
            // 2. Construct the attack block 'r'
            /// Formula: r <- (L | (R XOR u)) XOR 'y' [cite:: 237]
            /// L: Random filler
            /// R: Target padding pattern (i-1)
            /// u: Our guess
            /// y': The IV/Previous block from the captured session
            let mut r = vec![0u8; BLOCK_SIZE];
            // Fill L (random junk)
            for k in 0..(BLOCK_SIZE -i) {
                r[k] = random_byte();
            }
            // Construct (R XOR u) XOR y' for the suffix
            let pad_byte = (i - 1) as u8; // The padding byte value we want the oracle to see

            for k in 0..i {
                let idx = BLOCK_SIZE - i + k;
                let r_val = pad_byte ^ u[k]; // R XOR u
                r[idx] = r_val ^ y_prime[idx]; // ... XOR y'
            }

            // 3. Query Oracle with constructed IV 'r' and target block 'y'
            let duration = oracle.query(&r, &y);
            let t_j = duration.as_secs_f64() * 1000.0;

            /// 4. Update SPRT Log-Likelihood [cite: 121]
            /// LLR += ln( P(T|Valid) / P(T|Invalid))
            /// Using Gaussian PDF simplified ratio
            let numerator = (t_j - mu_w).powi(2) - (t_j - mu_r).powi(2);
            let denominator = 2.0 * sigma.powi(2);
            log_likelihood_ratio += numerator / denominator;
            // Avoid div by zero
            
            // 5. Check Thresholds (STOP predicate) [cite: 172]
            if log_likelihood_ratio > THRESHOLD_ACCEPT {
                return true; // Accepted: The oracle spent time, so padding was valid!
            }
            if log_likelihood_ratio < THRESHOLD_REJECT {
                return false; // Rejected: The oracle returned fast, padding was invalid.
            }
    
    }
}

/// The DecrypByte4 Algorithm [cite: 207]
/// Recovers one byte of the plaintext
fn decrypt_byte4(oracle: &MockOracle, known_suffix: &[u8], block_index: usize) -> u8 {
    // In a real attack, we would sort by likelihood (Dictionary Attack [cite: 256])
    // Here we iterate all 256 possibilities
    for candidate in 0..=255 {
        // Construct trial suffix : candidate | known_suffix
        let mut u = vec![candidate];
        u.extend_from_slice(known_suffix);

        // Check if this guess creates valid padding
        if check4(oracle, &u, block_index) {
            return candidate;
        }
    }
    panic!("Fail to decrypt byte - no candidate accepted");
}

/// The DecryptBlock4 Algorithm
/// Recovers a full block of plaintext.
fn decrypt_block4(oracle: &MockOracle, block_index: usize) -> Vec<u8> {
     let mut decrypted_block = Vec::new();

     // Iterate backwards from last byted to first [cite: 202]
     // The paper iterates i=1 to b, building the suffix.
     for _i in 1..=BLOCK_SIZE {
        let byte = decrypt_byte4(oracle, &decrypted_block, block_index);
        // Prepend the found byte
        decrypted_block.insert(0, byte);

        // Visual progress
        print!("{:02x} ", byte);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
     }
     println!();
     decrypted_block
}
fn main() {
    println!("--- CBC-PAD Timing Attack Simulation (Vaudenay/Canvel et al.) ---");

    let secret = "PASSWRD1"; // 8 chars = 1 block
    let oracle = MockOracle::new(secret);

    println!("Target Secret: [Redacted]");
    println!("Block Size: {} bytes", BLOCK_SIZE);
    println!("Starting Attack...\n:");

    let start_time = Instant::now();

    // Attack the first block (index 0)
    print!("Decrypting Block 0: ");
    let recovered_bytes = decrypt_block4(&oracle, 0);

    let recovered_string = String::from_utf8_lossy(&recovered_bytes);

    println!("\nAttack Complete in {:.2?}", start_time.elapsed());
    println!("Recovered Hex: {:02x?}", recovered_bytes);
    println!("Recovered Text: {}", recovered_string);

    assert_eq!(recovered_string, secret);
    println!("SUCCESS: Secret matches!");

}