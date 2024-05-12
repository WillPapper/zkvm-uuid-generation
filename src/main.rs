use std::time::Instant;
use uuid::Uuid;

pub fn main() {
    // Prepare for V3 UUID
    let (prove_generate_uuid_v3, verify_generate_uuid_v3) = guest::build_generate_uuid_v3();

    // Time the proof generation for V3 UUID
    let start_proof_v3 = Instant::now();
    let (output_v3, proof_v3) = prove_generate_uuid_v3(b"hello@example.com");
    let duration_proof_v3 = start_proof_v3.elapsed();

    // Time the verification for V3 UUID
    let start_verify_v3 = Instant::now();
    let is_valid_v3 = verify_generate_uuid_v3(proof_v3);
    let duration_verify_v3 = start_verify_v3.elapsed();

    // Output for V3 UUID
    println!("--- Version 3 UUID ---");
    println!(
        "Output: {}",
        Uuid::from_bytes(output_v3).hyphenated().to_string()
    );
    println!("Valid: {}", is_valid_v3);
    println!(
        "Time taken for proving: {} ms",
        duration_proof_v3.as_millis()
    );
    println!(
        "Time taken for verification: {} ms",
        duration_verify_v3.as_millis()
    );

    // Prepare for V5 UUID
    let (prove_generate_uuid_v5, verify_generate_uuid_v5) = guest::build_generate_uuid_v5();

    // Time the proof generation for V5 UUID
    let start_proof_v5 = Instant::now();
    let (output_v5, proof_v5) = prove_generate_uuid_v5(b"hello@example.com");
    let duration_proof_v5 = start_proof_v5.elapsed();

    // Time the verification for V5 UUID
    let start_verify_v5 = Instant::now();
    let is_valid_v5 = verify_generate_uuid_v5(proof_v5);
    let duration_verify_v5 = start_verify_v5.elapsed();

    // Output for V5 UUID
    println!("--- Version 5 UUID ---");
    println!(
        "Output: {}",
        Uuid::from_bytes(output_v5).hyphenated().to_string()
    );
    println!("Valid: {}", is_valid_v5);
    println!(
        "Time taken for proving: {} ms",
        duration_proof_v5.as_millis()
    );
    println!(
        "Time taken for verification: {} ms",
        duration_verify_v5.as_millis()
    );
}
