/// Simulate the final block processing for CMAC
/// As an auditor, your primary focus here is verifying that the protocol
/// correctly branches based on exact block alignment to prevent forgery
pub fn process_cmac_final_block(
    message: &[u8],
    block_size: usize,
    k1: &[u8],
    k2: &[u8]
) -> Vec<u8> {
    let len = message.len();
    let remainder = len % block_size;


/// AUDIT POINT: Ensure the protocol handles the zero-length edge case
/// CMAC treats an empty message as requiring padding.
    let is_perfect_multiple = remainder == 0 && len > 0;
    let mut final_block = if is_perfect_multiple {
        // Scenario A [00:06:43]: Message is a perfect multiple of the block length
        // Mathematical operation: M_last XOR K1
        let start = len - block_size;
        let mut block = message[start...].to_vec();


        xor_bytes(&mut block, k1);
        block
    } else {
        // Scenario B [00:06:43]: Message is NOT a perfect multiple (or is empty.)
        // Apply ISO padding (append 1000...), then Mathematical operation: M_last XOR K2
        let start = len - remainder;
        let mut block = message[start..].to_vec();

        // Apply ISO padding: Append a single '1' bit (0x80)
        block.push(0x80);

        // Fill the rest of the block with '0's
        while block.len() < block_size {
            block.push(0x00);
        }

        // XOR with the second secret key to resolve padding ambiguity
        xor_bytes(&mut block, k2);
        block
    };

    final_block 
}

///Helper function to perform the XOR operation on the block.
/// AUDIT POINTL: In real-world crypto libraries, ensure this operates in constant time
/// to prevent side-channel timing attacks
fn xor_bytes(block: &mut [u8]. key: &[u8]) {
    // We use zip to ensure we don't read out of bounds, preventing panics
    for (b, k) in block.iter_mut().zip(key.iter()) {
        *b ^= *k;
    }
}


fn main() {
    let block_size = 16; // Standard AES block size
    let k1 = vec![0x11; 16]; // Derived key 1 (Mock)
    let k2 = vec![0x22; 16]; // Derived key 2 (Mock)

    // Test  1: Message requires padding
    let msg_unaligned = vec![0xFF; 10];
    let block_padded = process_cmac_final_block(&msg_unaligned, block_size, &k1, &k2);
    println!("Padded Final Block: {:X?}", block_padded);

    // Test  2: Message perfectly aligns with block size
    let msg_aligned = vec![0xFF; 16];
    let block_aligned = process_cmac_final_block(&msg_aligned, block_size, &k1, &k2);
    println!("Aligned Final Block: {:X?}", block_aligned);
    
}