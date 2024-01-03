# Installation
## Using `cargo-generate` with vendored OpenSSL
However, you can also opt in to use a vendored OpenSSL version.
So that you don't have to have OpenSSL installed and built it on the spot.

this would require the following dependencies on your system, as documented by the [`openssl` crate]:
- A C compiler (`gcc`, for example)
- `perl` and `perl-core`
- `make`

```sh
cargo install cargo-generate --features vendored-openssl
```