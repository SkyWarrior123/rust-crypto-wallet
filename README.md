## app.rs
- Creating an enum for all the supported Blockchains
```rs
pub enum Blockchains {
	Bitcoin,
	Ethereum
}
```

- Two main `impls` in the `app.rs`:
* Implements the `FromStr` trait for the `Blockchains` enum, allowing conversion from a string `slice (&str)` to a `Blockchains` instance.
* Implements the `ToString` trait for the `Blockchains` enum, enabling conversion from a `Blockchains` instance to a `String`.