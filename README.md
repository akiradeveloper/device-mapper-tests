# device-mapper-tests

## Crates

- device-mapper-tests: The test framework. **Please make sure all tests for the framework completes in success.**
- writeboost-tests: Tests for [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost). You need to `modprobe dm-writeboost`.
- wb-command-tests: Tests for [dm-writeboost-tools](https://github.com/akiradeveloper/dm-writeboost-tools). You need to `modprobe dm-writeboost` and `cargo install` the dm-writeboost-tools.

## Requirements for the framework

These modules must be installed in the system to get the framework working.

- xfs, xfsprogs
- crypt target, cryptsetup
- CRYPTO_USER
- CRYPTO_USER_API_SKCIPHER

Run `cryptsetup benchmark -c aes-xts-plain64 -s 512` to make sure the cryptsetup is correctly installed.
