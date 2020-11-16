# Lingua Mzansi

This is a Q&D test of https://github.com/pemistahl/lingua-rs on South African languages.

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

## Sample Output

The initial commits tested this with a sentence "Our king is very wise", translated to Afrikaans, Sotho, Tswana, Xhosa and Zulu.

```rust
// test 1
let detected_language: Option<Language> = detector.detect_language_of("Kgosi ya rona e botlhale thata");
println!("The detected language is {:?}", detected_language);
assert_eq!(detected_language, Some(Tswana));

// test 2
let detected_language: Option<Language> = detector.detect_language_of("Morena oa rona o bohlale haholo");
println!("The detected language is {:?}", detected_language);
assert_eq!(detected_language, Some(Sotho));

// test 3
let detected_language: Option<Language> = detector.detect_language_of("Ons koning is baie wys");
println!("The detected language is {:?}", detected_language);
assert_eq!(detected_language, Some(Afrikaans));

// test 4
let detected_language: Option<Language> = detector.detect_language_of("Ukumkani wethu ulumke kakhulu");
println!("The detected language is {:?}", detected_language);
assert_eq!(detected_language, Some(Xhosa));

// test 5
let detected_language: Option<Language> = detector.detect_language_of("Inkosi yethu ihlakaniphe kakhulu");
println!("The detected language is {:?}", detected_language);
assert_eq!(detected_language, Some(Zulu));
```

The output of the run is:

```rust
     Running `target\release\lingua-mzansi.exe`
The detected language is Some(Tswana)
The detected language is Some(Sotho)
The detected language is Some(Afrikaans)
The detected language is Some(Xhosa)
The detected language is Some(Zulu)
```