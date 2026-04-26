use std::collections::HashMap;

// 1. THE ARCHITECTURE
// We define a 16-bit Substitution-Permutation Network (SPN) cipher structure.
// This uses four 4-bit S-boxes per round
const SBOX: [u8; 16] = [0xE, 0x4, 0xD, 0x1, 0x2, 0xF, 0xB, 0x8, 0x3, 0xA, 0x6, 0xC, 0x5, 0x9, 0x0, 0x7];
const INV_SBOX: [u8; 16] = [0xE, 0x3, 0x4, 0x8, 0x1, 0xC, 0xA, 0xF, 0x7, 0xD, 0x9, 0x6, 0xB, 0x2, 0x0, 0x5];

// A basic Bit Permutation layer to spread the statistical bias across multiple S-boxes
const PBOX: [u8; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];

// Fast  parity calculator using native CPU popcount
// Returns 0 if even number of 1s, 1 if odd.

#[inline(always)]
fn parity(val: u16) -> u16 {
    (val.count_ones() & 1) as u16
}

// --- PHASE 1: THE BIAS (Automated Vulnerability Discovery) ---

// Generates the Linear Approximation Table (LAT) for the S-Box.
// This mathematically proves exactly where the cipher leaks the most information.
fn generate_lat() -> (u8, u8, f64) {
    println!("--- PHASE 1: COMPUTING LINEAR APPROXIMATION TABLE (LAT) ---");
    let mut max_bias = 0.0;
    let mut best_mask_in = 0;
    let mut best_mask_out = 0;

    // Test all 256 possible combinations of Input Masks (1-15) and Output Masks (1-15)
    for mask_in in 1..16u8 {
        for mask_out in 1..16u8 {
            let mut matches = 0;

            // Test the equation across the entire 16-state universe of the S-box
            for x in 0..16u8 {
                let y = SBOX[x as usize];
                // Equation: (X * Mask_In) XOR (Y * Mask_Out) == 0
                let x_parity = parity((x & mask_in) as u16);
                let y_parity = parity((y & mask_out) as u16);

                if x_parity ^ y_parity == 0 {
                    matches += 1;
                }
            }

            // Calculate epsilon (e) from the probability: P = 0.5 + e
            let probability = matches as f64 / 16.0;
            let bias = (probability - 0.5).abs();


            if bias > max_bias {
                max_bias = bias;
                best_mask_in = mask_in;
                best_mask_out = mask_out;
            }
        }
    }

    println!("Vulnerability Discovered!");
    println!("Best Input Mask : 0x{:X}", best_mask_in);
    println!("Best Output Mask : 0x{:X}", best_mask_out);
    println!("Maximum Bias (e): {:.4}\n", max_bias);

    (best_mask_in, best_mask_out, max_bias)
}


// --- PHASE 2: THE EXPLOIT (Data Harvest) ---
// Simulates a 2-round 16-bit SPN cipher encryption.
fn encrypt_16bit(mut state: u16, keys: &[u16; 3]) -> u16 {
    for round in 0..2 {
        // 1. AddRoundKey
        state ^= keys[round];

        // 2. SubBytes (Apply S-box to each of the four 4-bit blocks)
        let mut sub_state = 0;
        for i in 0..4 {
            let shift = i * 4;
            let nibble = ((state >> shift) & 0xF) as u8;
            sub_state |= (SBOX[nibble as usize] as u16) << shift;
        }
        state = sub_state;

        // 3. PermuteBytes (Skip on the final round, standard SPN design)
        if round < 1 {
            let mut perm_state = 0;
            for i in 0..16 {
                if (state & (1 << i)) != 0 {
                    perm_state |= 1 << PBOX[i as usize];
                }
            }
            state = perm_state;
        }
    }
    // Final key whitening
    state ^ keys[2]
}

// Minimal Pseudo_Random Number Generator for data collection
struct Lcg { state: u32 }
impl Lcg {
    fn next_u16(&mut self) -> u16 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 16) as u16
    }               
}

// ---- PHASE 3: THE COLLAPSE (Matsui's Algorithm 2) ---

fn main() {
    // 1/ Dynamically find the fatal flaw in the S-box mathematics
    let (mask_in, mask_out, expected_bias) = generate_lat();

    // The Master Keys, We will attack the first 4 bits of the final round key (K3).
    let target_keys: [u16; 3] = [0x1234, 0x5678, 0x9ABC];
    let actual_target_subkey = (target_keys[2] & 0x000F) as u8; // 0xC

    let num_samples = 15_000;
    let mut rng = Lcg { state: 1337 };

    println!("--- PHASE 2: HARVESTING DATA ---");
    println!("Generating {} Known Plaintext/Ciphertext pairs...", num_samples);
    let mut pairs = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        let p = rng.next_u16();
        let c = encrypt_16bit(p, &target_keys);
        pairs.push((p, c));
    }
    println!("Data harvest complete.\n");


    println!("--- PHASE 3: MATSUI's ALGORITHM 2 (Extracting Subkey) ---");
    println!("Targeing the lowest 4 bits of the final round key...");

    let mut best_guess = 0;
    let mut max_observed_bias = 0.0;

    // We only need to brute-force the 4 bits of the final key that correspond
    // to the specific S-box we are tracking (the lowest nibble);.
    for subkey_guess in 0..16u8 {
        let mut match_count = 0;

        for &(p, c) in &pairs {
            // Extract the lowest 4 bits fo the Ciphertext and Plaintext
            let c_nibble = (c & 0x000F) as u8;
            let p_nibble = (p & 0x000F) as u8;
            
            // PARTIAL DECRYPTION: Reverse the last key addition and substitution
            // for just this one specific S-box using our subkey guess.
            let z = c_nibble ^ subkey_guess;
            let y_guess = INV_SBOX[z as usize];

            // Apply our optimal masks discovered in Phase 1
            let p_parity = parity((p_nibble & mask_in) as u16);
            let y_parity = parity((y_guess & mask_out) as u16);

            if p_parity ^ y_parity == 0 {
                match_count += 1;
            }
        }
        let probability = match_count as f64 / num_samples as f64;
        let bias = (probability - 0.5).abs();

        if bias > max_observed_bias {
            max_observed_bias = bias;
            best_guess = subkey_guess;
        }
    }
    println!("Attack Execution Complete.");
    println!("----------------------------------------------------------------------------");
    println!("Expected Bias from LAT : {:.4}", expected_bias);
    println!("Max Observed Bias      : {:.4}", max_observed_bias);
    println!("Guessed Subkey Nibble  : 0x{:X}", best_guess);
    println!("Actual Subkey Nibble   : 0x{:X}", actual_target_subkey);

    if best_guess == actual_target_subkey {
        println!("Result: SYSTEM COMPROMISED. The subkey was successfully extracted.");
    } else {
        println!("Result: Attack failed. Cipher diffusion held.");
    }
}
