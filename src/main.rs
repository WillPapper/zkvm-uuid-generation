use uuid::Uuid;

pub fn main() {
    let (prove_generate_uuid, verify_generate_uuid) = guest::build_generate_uuid();

    let (output, proof) = prove_generate_uuid(b"hello@example.com");
    let is_valid = verify_generate_uuid(proof);

    println!(
        "output: {}",
        Uuid::from_bytes(output).hyphenated().to_string()
    );
    println!("valid: {}", is_valid);
}
