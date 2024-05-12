# zkvm-uuid-generation

Verifiable UUID generation using A16Z Crypto's [Jolt zkVM](https://github.com/a16z/jolt)

## Overview

UUID generation typically takes place server-side, because clients cannot be trusted to provide correct UUID values that avoid collisions with other UUIDs. There are many cases, however, when you want to have UUID generation client-side (e.g. for peer-to-peer networks with no central server, or for private data that requires a UUID but does not want to reveal the data itself).

This project allows for verifiable UUID generation using Jolt's zkVM. Clients can verify that a UUID was generated correctly, without requiring the generating client to reveal the contents of the UUID.

Generation for strings is relatively fast, with proving taking 350ms - 500ms for short strings (e.g. email addresses) and verification taking 100ms - 150ms. Using UUID V5 over UUID V3 for better collision reistance adds around 50ms to proving and around 20ms to verification. For longer strings, UUID V3 can take 650ms for proving and 200ms for verification, while UUID V5 takes 1000ms for proving and 250ms for verification.

You can modify the strings in use in [src/main.rs](src/main.rs) and evaluate timestamps as desired.

## Quickstart

1. Install Jolt:

```
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
```

2. Run the following command inside the root directory (the same one as this README):

```
cargo run --release
```

For more information, see the [Jolt Quickstart](https://jolt.a16zcrypto.com/usage/quickstart.html)
