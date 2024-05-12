#![no_main]

use uuid::Uuid;

// Helper function to convert UUID to a fixed-size byte array
fn uuid_to_fixed_array(uuid: Uuid) -> [u8; 16] {
    let uuid_bytes = uuid.as_bytes();
    let mut uuid_fixed_size = [0u8; 16];
    uuid_fixed_size.copy_from_slice(uuid_bytes);
    uuid_fixed_size
}

// Generates a Version 3 UUID using the DNS namespace and the provided name
#[jolt::provable]
fn generate_uuid_v3(name: &[u8]) -> [u8; 16] {
    // Use UUID v3 for scenarios where performance is crucial due to its less
    // computationally intensive MD5 hashing, adhering to the strict resource
    // requirements of the zkVM:
    // https://stackoverflow.com/a/20342413
    let uuid = Uuid::new_v3(&Uuid::NAMESPACE_DNS, name);
    uuid_to_fixed_array(uuid)
}

// Generates a Version 5 UUID using the DNS namespace and the provided name
#[jolt::provable]
fn generate_uuid_v5(name: &[u8]) -> [u8; 16] {
    // Use UUID v5 when collision resistance is critical due to its use of
    // SHA-1, providing enhanced security over MD5:
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, name);
    uuid_to_fixed_array(uuid)
}
