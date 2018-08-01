Step 2.7: Cryptography
======================

While [Rust] does not have currently The Cryptographic Library, its ecosystem contains a bunch of well implemented (and still maturing) crates for different purposes. Let's overview them below.




## Encryption and signing


### [ring]

[ring] library is the first candidate for being The Cryptographic Library for [Rust] in future. It implements a core set of cryptographic operations exposed via an easy-to-use (and hard-to-misuse) API. It inherits its cryptography primitives from [BoringSSL] library.

[ring] is focused on general-purpose cryptography. If you need just raw cryptography primitives - that is the way to go. Use it when you need to create:
- digital signature;
- simply encrypt plain data;
- key derivation;
- and so on...

If you need more high-level implementations (like WebPKI [X.509] certificate validation, or cryptographic protocols like [TLS], [SSH]) consider to use other crates (which are often built on top of [ring]).


### [dalek]

While [ring] is focused to provide general-purpose cryptography primitives, [dalek] crates provide only few, but are focused to implement best theoretical primitives.

If you're going to build something that uses just some high-end cryptographic primitives (like using [Curve25519] for signing and verification) you should give [dalek] a try.




## Hashing


### Raw hash functions

The basic collection of raw [cryptographic hash functions][1] is introduced in [RustCrypto/hashes] crates collection.

__DO NOT use them for password hashing!__ Consider to use some password hashing algorithm instead ([Argon2], [bcrypt], [scrypt] or [PBKDF2]).


### Password hashing

There is the similar [RustCrypto/password-hashing] crates collection for password hashing.

However, it lacks implementation for [Argon2] and [bcrypt] algorithms, so those [should be found][2] and chosen on your choice. For [Argon2] the [argonautica] crate seems to be the most mature one.




## Constant-time comparision

For constant-time comparision in [Rust] consider to use [subtle] crate from [dalek].




## Cryptographically secure randomness

For [cryptographically secure random values][3] consider to use any random number generator from [rand] crate that implements `CryptoRng` marker trait.

[ring] has also its own `rand::SystemRandom` type which implements secure random number generator where the random values come directly from the underlying operating system.




## TLS / SSL

For [TLS] usage [Rust] ecosystem currently has two common solutions:


### [native-tls]

[native-tls] crate is an abstraction over platform-specific [TLS] implementations. It uses [SChannel] on Windows (via [schannel] crate), Secure Transport on OSX (via [security-framework] crate), [OpenSSL] on all other platforms (via [openssl] crate), and provides an unified interface for using this libraries.

While this solution requires external non-[Rust] libraries to be present, it's a stable solution based on production-grade [TLS] implementations.


### [rustls]

[rustls] crate is a pure-[Rust] implementation of [TLS]. It's built on top of [ring] and [webpki] crates.

Despite the fact it's quite a feature rich solution, it [lacks good support for old and legacy cryptography][4] and has no stable version yet. Consider to use it when the legacy is no concern for you.




## Task

Implement three functions:
1. `get_file_name_and_hash()` which for a [given file](pic.jpg) returns a new generated unique name and a hash of its content.
2. `encrypt_file_content()` which produces a new encrypted version of a [given file](pic.jpg).
3. `check_key_is_correct()` which checks if a secret from previous function is correct by its fingerprint.





[Rust]: https://www.rust-lang.org
[argonautica]: https://crates.io/crates/argonautica
[dalek]: https://dalek.rs
[native-tls]: https://crates.io/crates/native-tls
[openssl]: https://crates.io/crates/openssl
[rand]: https://crates.io/crates/rand
[ring]: https://crates.io/crates/ring
[rustls]: https://crates.io/crates/rustls
[schannel]: https://crates.io/crates/schannel
[security-framework]: https://crates.io/crates/security-framework 
[subtle]: https://crates.io/crates/subtle
[webpki]: https://crates.io/crates/webpki
[RustCrypto/hashes]: https://github.com/RustCrypto/hashes
[RustCrypto/password-hashing]: https://github.com/RustCrypto/password-hashing
[Argon2]: https://en.wikipedia.org/wiki/Argon2
[bcrypt]: https://en.wikipedia.org/wiki/Bcrypt
[BoringSSL]: https://github.com/google/boringssl
[Curve25519]: https://en.wikipedia.org/wiki/Curve25519
[OpenSSL]: https://en.wikipedia.org/wiki/OpenSSL
[PBKDF2]: https://en.wikipedia.org/wiki/PBKDF2
[SChannel]: https://en.wikipedia.org/wiki/Security_Support_Provider_Interface
[scrypt]: https://en.wikipedia.org/wiki/Scrypt
[SSH]: https://en.wikipedia.org/wiki/Secure_Shell
[TLS]: https://en.wikipedia.org/wiki/Transport_Layer_Security
[X.509]: https://en.wikipedia.org/wiki/X.509

[1]: https://en.wikipedia.org/wiki/Cryptographic_hash_function
[2]: https://crates.io/search?q=argon2
[3]: https://crypto.stackexchange.com/questions/39186/what-does-it-mean-for-a-random-number-generator-to-be-cryptographically-secure
[4]: https://docs.rs/rustls/#non-features
