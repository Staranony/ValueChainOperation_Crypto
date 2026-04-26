use k256::{ProjectivePoint, Scalar};
use k256::elliptic_curve::group::GroupEncoding; // For to_bytes()
use k256::elliptic_curve::Field; // For Scalar::random
use rand_core::OsRng;
use std::ops::Mul;

// =====================================================================
// PROTOCOL ENGINEER TOOLKIT: PEDERSON COMMITMENT
// Formula: C = v*G + r*H
// =====================================================================


fn main() {
    println!("--- Starting ZK commitment Audit ---");

    // 1. Setup the Protocol Parameters (Public Constants)
    // G is usually the standard generator of the curve.
    let G = ProjectivePoint::GENERATOR;

    // H must be a point where nobody knows the "discrete log" relative to G.
    // In production, this is generated via "Nothing Up My Sleeve" numbers.
    // Here, for simplicity, we derive it deterministically (insecure for prod, ok for demo).
    let H = ProjectivePoint::GENERATOR.mul(Scalar::from(12345u64));

    // 2. The User's Secret Data
    let vote_value = Scalar::from(1u64); // 1 = "YES"

    println!("User is voting: YES (Value: 1)");
    
    // =====================================================================
    // SCENARIO A: The "ECB" Mistake (No Randomness)
    // C = v*G + 0*H
    // =====================================================================

    let r_bad = Scalar::ZERO;
    let commitment_bad_1 = (G * vote_value) + (H * r_bad);
    let commitment_bad_2 = (G * vote_value) + (H * r_bad);

    println!("\n[BAD] Determinstic Commitments (Like ECB):");
    println!("Commit 1: {}", hex::encode(commitment_bad_1.to_bytes()));
    println!("Commit 2: {}", hex::encode(commitment_bad_2.to_bytes()));

    if commitment_bad_1 == commitment_bad_2 {
        println!(">> AUDIT FAIL: Commitments are identical. Observer knows you voted the same way twice.");
    }

    // =====================================================================
    // SCENARIO B: The "ZK" Fix (With Randomness / Blinding)
    // C = v*G + r*H
    // =====================================================================

    // Attempt 1 with random blinding factor
    let r1 = Scalar::random(&mut OsRng);
    let commitment_secure_1 = (G * vote_value) + (H * r1);
    
    // Attempt 2 with NEW random blinding factor
    let r2 = Scalar::random(&mut OsRng);
    let commitment_secure_2 = (G * vote_value) + (H* r2);

    println!("\n[GOOD] Pedersen Commitments (With Blinding):");
    println!("Commit 1: {}", hex::encode(commitment_secure_1.to_bytes()));
    println!("Commit 2: {}", hex::encode(commitment_secure_2.to_bytes()));

    if commitment_secure_1 != commitment_secure_2 {
        println!(">> AUDIT PASS: Commitments look completely different, despite hiding the same value.");

    }
}
