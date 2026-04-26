fn simulate_ecb_encrypt(block: &[u8; 4], key: u32) -> [u32; 4] {
    // In ECB, the encryption function is deterministic.
    // The same input block + same key ALWAYS yields the same output.
    // We simulate a complex cipher with a simple XOR for demonstration.
    block.map(|byte| (byte as u32) ^ key)
}

fn main() {
    let secret_key = 0xDEADBEEF;

    // Simulating a row of pixels in an image:
    // White background, White background, Black outline, White background
    let image_data_blocks =  [
        [255, 255, 255, 255], // Block 1
        [255, 255, 255, 255], // Block 2
        [0,   0,   0,   0  ], // Block 3
        [255, 255, 255, 255], // Block 4
    ];

    println!("Encrypted blockss output:");
    for (i, block) in image_data_blocks.iter().enumerate() {
        let ciphertext = simulate_ecb_encrypt(block, secret_key);
        println!("Block {}: {:?}", i + 1, ciphertext);
    }
}


// ###############################################################


fn simulate_ecb_encrypt(block: &[u8; 4], key: u32) -> [u32; 4] {
    block.map(|byte| (byte as u32) ^ key)
}

// Let's introduce a new encryption mode. Notice the third parameter.
fn simulate_chained_encrypt(
    block: &[u8; 4],
    key: u32,
    previous_ciphertext: &[u32; 4]
) -> [u32; 4] {
    let mut output = [0; 4];
    for i in 0..4 {
        // We mix the plaintext with the previous ciphertext BEFORE applying the key
        let mixed_byte = (block[i] as u32) ^ previous_ciphertext[i];
        output[i] = mixed_byte ^ key;
    }
    output
}

fn main() {
    let secret_key = 0xDEADBEEF;
    let iv = [0x1234, 0x5678, 0x9ABC, 0xDEF0]; // Initialization Vector

    // Two identical blocks of white pixels
    let block_1 = [255, 255, 255, 255];
    let block_2 = [255, 255, 255, 255];

    // ECB Mode
    let ecb_out_1 = simulate_ecb_encrypt(&block_1, secret_key);
    let ecb_out_2 = simulate_ecb_encrypt(&block_2, secret_key);

    // Chained Mode
    let chained_out_1 = simulate_chained_encrypt(&block_1, secret_key, &iv);
    let chained_out_2 = simulate_chained_encrypt(&block_2, secret_key, &chained_out_1);

    // If you ran this, ecb_out_1 == ecb_out_2.
    // But chained_out_1 != chained_out_2
}