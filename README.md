# Lombok Rust
[![](https://docs.rs/lombok/badge.svg)](https://docs.rs/lombok/)
[![](https://img.shields.io/crates/v/lombok.svg)](https://crates.io/crates/lombok)
[![](https://img.shields.io/crates/d/lombok.svg)](https://crates.io/crates/lombok)


[Lombok](https://projectlombok.org) port for Rust.

## Why?

Just because I can!

This crate is not actually the must-have one for development in Rust (unlike Java world), but if you find it useful - it
would be great. Anyway - boilerplate sucks, so get some proc macros stuff for decrease it.

Any feedback is appreciated.

## Implementation list so far

- [x] `@Getter` - `#[derive(Getter)]`/`#[derive(GetterMut)]`
- [x] `@Setter` - `#[derive(Setter)]`
- [x] `@EqualsAndHashCode` - `#[derive(EqualsAndHashCode)]`
- [x] `@ToString` - `#[derive(ToString)]`
- [x] `@Data` - `#[derive(Data)]`
- [x] `@NoArgsConstructor` - `#[derive(NoArgsConstructor)]`
- [x] `@AllArgsConstructor` - `#[derive(AllArgsConstructor)]`
- [x] `@Builder` - `#[derive(Builder)]`

## Usage

Update `Cargo.toml`

```toml
[dependencies]
lombok = "0.3"
```

Source code usage examples you can see [in tests](./tests/tests.rs).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE.md) or [MIT license](LICENSE-MIT.md) at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
