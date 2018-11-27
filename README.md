# tyname - Type names on stable Rust

|       Docs       |       Crates.io      |
|:----------------:|:--------------------:|
| [![docs][0]][1]   | [![crates][2]][3] |

[0]:  https://docs.rs/tyname/badge.svg
[1]: https://docs.rs/tyname
[2]: https://img.shields.io/crates/v/tyname.svg
[3]: https://crates.io/crates/tyname

**WORKS ON STABLE RUST**

Retrieve type names during program execution on **stable** Rust.

Other solutions in the Rust ecosystem use the unstable `core::intrinsics::type_name` API.

## Examples

### Use

Works for every built-in type.

```rust
assert_eq!(type_name::<()>(), String::from("()"));
assert_eq!(type_name::<i32>(), String::from("i32"));
assert_eq!(type_name::<[u8; 32]>(), String::from("[u8; 32]"));
```

Works for tuples up to 10 different fields.

```rust
assert_eq!(
	type_name::<(i8, i16, i32, i64, i128)>(),
	String::from("(i8, i16, i32, i64, i128)")
);
```

Works on structs.

```rust
assert_eq!(
	type_name::<Vec<u8>>(), String::from("Vec<u8>")
);
assert_eq!(
	type_name::<Result<i32, String>>(),
	String::from("Result<i32, String>")
);
```

Works on function pointer types.

```rust
assert_eq!(
	type_name::<fn(i32, f32) -> bool>(),
	String::from("fn(i32, f32) -> bool")
);
assert_eq!(
	type_name::<fn()>(),
	String::from("fn() -> ()")
);
```

### Implement

The `TypeName` trait is used for retrieving the names.
Every type that implements it can be used.
This library already implements the most common types for the users

Users can implement it manually for their own types, too.

**Note:** A derive functionality is planned but not yet crafted.

```rust
/// The type we want to make work for the `TypeName` trait
struct Foo<T1, T2> { a: T1, b: T2 }

/// The manual implementation.
impl<T1, T2> crate::TypeName for Foo<T1, T2>
where
	T1: TypeName,
	T2: TypeName,
{
	fn write_type_name<W>(w: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write
	{
		w.write_str("Foo<")?;
		T1::write_type_name(w)?;
		w.write_str(", ")?;
		T2::write_type_name(w)?;
		w.write_char('>')
	}
}

fn main() {
	assert_eq!(
		type_name::<Foo<bool, char>>(),
		String::from("Foo<bool, char>")
	);
}
```

## Future

With a `#[derive(TypeName)]` functionality it will be possible to implement this
for custom types such as the `struct Foo` above like the following.

```rust
#[derive(TypeName)]
struct Foo<T1, T2>{
	a: T1,
	b: T2,
}
```

Done.

## Short comings

- No `#[derive(TypeName)]` functionality provided, yet.
- Requires computation at run-time compared to the `core::intrinsics::type_name` API
- Cannot print out the paths to the type, e.g. the `std::result::` in `std::result::Result<T, E>`.
- Currently has problems with function pointer types returning the unit type,
  e.g. `fn()` will print `fn() -> ()` instead.

## License

Licensed under either of

 * Apache license, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Dual licence: [![badge][license-mit-badge]](LICENSE-MIT) [![badge][license-apache-badge]](LICENSE-APACHE)


[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-apache-badge]: https://img.shields.io/badge/license-APACHE-orange.svg

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
