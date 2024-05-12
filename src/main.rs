use std::time::Instant;
use uuid::Uuid;

// Function to handle the generation and verification of UUIDs
fn timestamp_uuid_version<F, G, H, Proof>(build_fn: F, version_label: &str, name: &[u8])
where
    F: FnOnce() -> (G, H),
    G: FnOnce(&[u8]) -> ([u8; 16], Proof),
    H: FnOnce(Proof) -> bool,
{
    // Build the generation and verification functions
    let (prove_generate_uuid, verify_generate_uuid) = build_fn();

    // Time the proving
    let start_prove = Instant::now();
    let (output, proof) = prove_generate_uuid(name);
    let duration_prove = start_prove.elapsed();

    // Time the verification
    let start_verification = Instant::now();
    let is_valid = verify_generate_uuid(proof);
    let duration_verification = start_verification.elapsed();

    // Convert output to Uuid and format it
    let uuid = Uuid::from_bytes(output).hyphenated().to_string();

    // Output results
    println!("--- {} ---", version_label);
    println!("Output: {}", uuid);
    println!("Valid: {}", is_valid);
    println!("Time taken for proving: {} ms", duration_prove.as_millis());
    println!(
        "Time taken for verification: {} ms",
        duration_verification.as_millis()
    );
}

pub fn main() {
    // Handle V3 UUID
    timestamp_uuid_version(
        guest::build_generate_uuid_v3,
        "Version 3 UUID",
        b"hello@example.com",
    );

    // Handle V5 UUID
    timestamp_uuid_version(
        guest::build_generate_uuid_v5,
        "Version 5 UUID",
        b"hello@example.com",
    );
}
