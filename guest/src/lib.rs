#![no_main]

use uuid::Uuid;

#[jolt::provable]
fn generate_uuid() -> String {
    // uuid v3 is used due to the very strict resource requirements of the zkVM:
    // https://stackoverflow.com/a/20342413
    Uuid::new_v3(&Uuid::NAMESPACE_DNS, b"hello@example.com").hyphenated().to_string()
}

#[jolt::provable]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}
