// A Demonstration of Linear Cryptanalysis targeting a 2-round SPN cipher
// using a 4-bit block size and a mathmatically flawed S-Box.
// 1. THE FLAWED S-BOX
// This os S-box (from standard crpytanalysis tutorials) has a known linear bias 
// specifically, Input Mask 0xB (1011) and Output Mas 0x5 (0101) hold true
// with a probability significantly far from 50%.

const SBOX: [u8; 16] = [0xE, 0x4, 0xD, 0x1, 0x2, 0xF, 0xB, 0x8, 0x3, 0xA, 0x6, 0xC, 0x5, 0x9, 0x0, 0x7];
const INV_SBOX: [u8; 16] = [0xE, 0x3, 0x4, 0x8, 0x1, 0xC, 0xA, 0xF, 0x7, 0xD, 0x9, 0x6, 0xB, 0x2, 0x0, 0x5];

// Helper: Calculate the parity (XOR sum) of the bits.
fn parity(mut v: u8) -> u8 {
    let mut p = 0;
    while v > 0 {
        p ^= v & 1;
        v >>= 1;
    }
    p
}

// 2. THE VULNERABLE CIPHER
// A simple 2-round cipher: Key Mix -> Substitute -> Key Mix -> Substitute
fn encrypt(p: u8, k1: u8, k2: u8) -> u8 {
    let x = p ^ k1;             // Round 1 Key Mixing
    let y = SBOX[x as usize];   // Round 1 Substitution
    let z = y ^ k2;             // Round 2 Key Mixing
    SBOX[z as usize]                // Round 2 Substitution (Ciphertext)
}

// Minimal Linear Congruential Generator for reproducible random plaintexts
struct Lcg { state: u32 }
impl Lcg {
    fn next_4bit(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        ((self.state >> 24) & 0x0F) as u8
    }
}

fn main() {
    // The secret keys the attacker is trying to find (4-bit keys: 0 to 15)
    let secret_k1: u8 = 0xA; // 1010
    let secret_k2: u8 = 0x7; // 0111

    let num_samples = 5000;
    let mut rng = Lcg { state: 42 };

    // Step 1: The Exploit - Gather Known Plaintext/Ciphertext pair
    println!("Gathering {} Plaintext/Ciphertext pairs...", num_samples);
    let mut pairs = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        let p = rng.next_4bit();
        let c = encrypt(p, secret_k1, secret_k2);
        pairs.push((p, c));
    }

    // Step 2: The Collapse - Attack the last round key (K2)
    let mask_in = 0xB;  // The biased input bits (1011)
    let mask_out = 0x5;     // The biased output bits (0101)

    let mut best_guess = 0;
    let mut max_bias = 0.0;

    println!("\nAnalyzing statistical biases for all possible K2 guesses:");
    println!("----------------------------------------------------------");
    for k2_guess in 0..16 {
        let mut match_count = 0;

        for &(p, c) in &pairs {
            // Reverse the last substitution using our guessed key
            // Z = INV_SBOX[C], therefore Y_guess = Z ^ k2_guess
            let z = INV_SBOX[c as usize];
            let y_guess = z ^ k2_guess;

            // Apply the linear approximation equation to the first round:
            // Parity(P & Mask_In) ^ Parity(Y_guess & Mask_Out) == 0
            let p_parity = parity(p & mask_in);
            let y_parity = parity(y_guess & mask_out);

            if p_parity ^ y_parity == 0 {
                match_count += 1;
            }
        }

        // Calculate how far the probability deviates from 50% (0.5)
        let probability = match_count as f64 / num_samples as f64;
        let bias = (probability - 0.5).abs();

        println!("K2 Guess: {:02X} | Match Prob: {:.4} | Bias: {:.4}", k2_guess, probability, bias);

        // The correct key will trigger the fundamental mathematical flaw in the S-Box,
        // resulting in the largest deviation (bias) from 50%.
        if bias > max_bias {
            max_bias = bias;
            best_guess = k2_guess;
        }
    }

    println!("------------------------------------------------------------------");
    println!("Attack Complete!");
    println!("Higheset Bias      : {:.4} (Significantly > 0.0)", max_bias);
    println!("Guessed Key 2      : 0x{:X}", best_guess);
    println!("Actual Key 2       : 0x{:X}", secret_k2);

    if best_guess == secret_k2 {
            println!("Result: FATAL COLLAPSE. Secret subkey extracted via statistical bias.");
    }
}









