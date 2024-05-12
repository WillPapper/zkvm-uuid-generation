#![no_main]

use uuid::Uuid;

// Generates a Version 3 UUID using the DNS namespace and the provided name
#[jolt::provable]
fn generate_uuid(name: &[u8]) -> [u8; 16] {
    // uuid v3 is used due to the very strict resource requirements of the zkVM:
    // https://stackoverflow.com/a/20342413
    // name is converted to bytes to be accepted by the Uuid::new_v3 function
    let uuid = Uuid::new_v3(&Uuid::NAMESPACE_DNS, name);
    let uuid_bytes = uuid.as_bytes();
    let mut uuid_fixed_size = [0u8; 16];
    uuid_fixed_size.copy_from_slice(uuid_bytes);
    uuid_fixed_size
}
