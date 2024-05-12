use uuid::Uuid;

pub fn main() {
    // Generate a V3 UUID and prove that it is valid:
    let (prove_generate_uuid_v3, verify_generate_uuid_v3) = guest::build_generate_uuid_v3();

    let (output_v3, proof_v3) = prove_generate_uuid_v3(b"hello@example.com");
    let is_valid_v3 = verify_generate_uuid_v3(proof_v3);

    println!(
        "v3 output: {}",
        Uuid::from_bytes(output_v3).hyphenated().to_string()
    );
    println!("v3 valid: {}", is_valid_v3);

    // Generate a V5 UUID and prove that it is valid:
    let (prove_generate_uuid_v5, verify_generate_uuid_v5) = guest::build_generate_uuid_v5();

    let (output_v5, proof_v5) = prove_generate_uuid_v5(b"hello@example.com");
    let is_valid_v5 = verify_generate_uuid_v5(proof_v5);

    println!(
        "v5 output: {}",
        Uuid::from_bytes(output_v5).hyphenated().to_string()
    );
    println!("v5 valid: {}", is_valid_v5);
}
