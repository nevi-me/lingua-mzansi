# Lingua Mzansi

This is a Q&D test of [https://github.com/pemistahl/lingua-rs](github:pemistahi/lingua-rs) on South African languages.

I initially ran tests on a single sentence translated to the languages that I know.
I'd like to test out subtle differences between Zulu and Xhosa, Tswana and Sotho (I suspect Sepedi is classified as 'Sotho' too).

## Running

You might have to be familiar with the Rust language, but to make things easy:

1. Go to [rustup.rs](rustup.rs), and follow the guide on installing Rust (stable is fine)
2. Clone this repo
3. Run `cargo run --release`

`cargo` is the package manager and code builder (think of it as equivalent to `npm` or `cmake`).
The `--release` flag tells the compiler to optimise the binary being created, otherwise this might feel slow.

Enjoy!