# auid

64 bit timestamp-first unique identifier

## Usage

```rust
let id = Uid::new();
println!("{id}");
```

## Features

| name     | description                                                                   | default? |
|:---------|:------------------------------------------------------------------------------|:--------:|
| `serde`  | [`serde`](https://docs.rs/serde) support                                      |    ✓     |
| `base16` | convert from/to base16 using [`faster-hex`](https://docs.rs/faster-hex)       |    𐄂    |
| `hex`    | alias for `base16`                                                            |          |
| `base32` | convert from/to base32 using [`data-encoding`](https://docs.rs/data-encoding) |    𐄂    |
| `base58` | convert from/to base58 using [`bs58`](https://docs.rs/bs58)                   |    𐄂    |
| `base64` | convert from/to base64 using [`data-encoding`](https://docs.rs/data-encoding) |    𐄂    |

## License

Licensed under [☕ Coffee License](https://coffee-license.org).
