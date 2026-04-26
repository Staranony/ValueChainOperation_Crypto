use aes::cipher::{KeyInit, BlockEncrypt, generic_array::GenericArray};
use aes::cipher::consts::U16;
use aes::Aes128;
use std::str;

// =====================================================================
// 1. THE VULNERABLE PROTOCOOL (The "Challenger" in your slide)
// =====================================================================
struct VulnerableOracle {
    key: GenericArray<u8, U16>,
}
impl VulnerableOracle {
    fn new() -> Self {
        // In realitym this key is secret and random.
        // We use a fixed key here for the sake of the demo,
        // but normally this is hidden inside the "black box".
        let key = GenericArray::from([0u8; 16]);
        Self { key }
    }
    // This Function implements AES-ECB (The "bad" logic)
    // It encrpyts block by block without chaining.
    fn encrypt_ecb(&self, plaintext: &[u8]) -> Vec<u8> {
        let cipher = Aes128::new(&self.key);
        let mut buffer = plaintext.to_vec();
    

    // SECURITY FLAW: ECB Mode
    // We are iterating over 16-byte blocks and encrypting them independently.
        for chunk in buffer.chunks_mut(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            chunk.copy_from_slice(&block);
            }
            buffer
        }

// The "Challenger" from the slide:
// Takes two messages, picks one (conceptually), and returns ciphertext.
// In the slide, the attacker sends m0 and m1.
// Here, we just return the encryption of the message the attacker wants to test.
        pub fn challenge(&self, message: &str) -> Vec<u8> {
            // Pad message to 16 bytes (PKCS7 is standard, but we'll use simple zero padding for simplicity)
            let mut bytes = message.as_bytes().to_vec();
            while bytes.len() % 16 != 0 {
                bytes.push(0);
            }
            self.encrypt_ecb(&bytes)
        }
}

// =====================================================================
// 2. THE AUDITOR / ADVERSARY (You)
// =====================================================================
fn main() {
    // Step 1: Initialize the vulnerable system
    let oracle = VulnerableOracle::new();

    println!("--- Starting IND_CPA Audit on ECB Mode ---");
    
    // Step 2: Prepare the Trap (The Attacker's Strategy)
    // As per the slide:
    // m0 = "Hello World" (Two distinct blocks logic, but actually "Hello World......" is one block)
    // To match the slide's logic of "Two blocks", we need 32 bytes.

    // Slide Case m1:"Hello Hello" -> Represent a Repeating Pattern.
    // Let's make 32-byte messages (2 blocks of 16 bytes).

    // Message A: Different Blocks (High Entropy)
    // "Block 1...........Block 2.............."
    let m0 = "A_Secret_Message_With_Dff_Parts";

    // Message B: Identical Blocks (Low Entropy)
    // "Block 1...........Block 1.............."
    let m1 = "Repeated_PatternRepeated_Pattern";

    println!("Attacker sends m0: {}", m0);
    println!("Attacker sends m1: {}", m1);

    // Step 3: Get Ciphertexts (The Oracle encrypts them)
    let _c0 = oracle.challenge(m0);
    let c1 = oracle.challenge(m1);

    // Step 4: The Audits (Analysis)
    // The slides says: If c1 == c2 (meaning block 1 == block 2), output 0 (or 1).

    println!("\nAnalyzing Ciphertext for m1 (Repeated Pattern)...");
    print_blocks(&c1);

    let is_ecb_detected = detect_ecb(&c1);

    if is_ecb_detected {
        println!("\n[CRITICAL VULNERABILITY FOUND] The protocol is using ECB mode.");
        println!("Reason: Identical plaintext blocks produced identical ciphertext blocks.");
        println!("Proof: Block 0 matches Block 1.");

    } else {
        println!("\n[PASS] Semantic security appears intact.");
    }

}


// The "Audit Tool" function
// Checks if any two blocks in the ciphertext are identical
fn detect_ecb(ciphertext: &[u8]) -> bool {
    let chunks: Vec<&[u8]> = ciphertext.chunks(16).collect();
    for i in 0..chunks.len() {
        for j in i + 1..chunks.len() {
            if chunks[i] == chunks[j] {
                return true; // MATCH FOUND!
            }
        }
    }
    false
}

fn print_blocks(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        println!("Block {}: {}", i, hex::encode(chunk));
    }
}
