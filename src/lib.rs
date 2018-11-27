//! Retrieve type names during program execution on **stable** Rust.

#![doc(html_root_url = "https://docs.rs/crate/tyname/0.1.0")]

#[cfg(test)]
mod tests;

use std::fmt::Write;

/// The result type for this crate.
pub type Result = std::fmt::Result;

/// Types that implement this trait can write their name.
pub trait TypeName {
	/// Applies the keccak hash of `self` for the given keccak hasher.
	fn write_type_name<W>(writer: &mut W) -> Result
	where
		W: Write;
}

/// Returns the name of the given type.
pub fn type_name<T>() -> String
where
	T: TypeName + ?Sized
{
	let mut buffer = String::new();
	T::write_type_name(&mut buffer)
		.expect("[tyname::type_name] Encountered error while writing type name");
	buffer
}

macro_rules! impl_tuple_signature_hash {
	// Specialization for the unit type (void)
	( ) => {
		impl TypeName for () {
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str("()")
			}
		}
	};
	// Specialization for unary-tuples
	( $head:ident ) => {
		impl<$head> TypeName for ($head,)
		where
			$head: TypeName,
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str("(")?;
				$head::write_type_name(w)?;
				// Comma needed here to differentiate between
				// parenthesized expressions and unary-tuples
				w.write_str(",)")
			}
		}

		impl_tuple_signature_hash!();
	};
	// Impl for generic tuples with at least two elements
	( $head:ident $($tail:ident)+ ) => {
		impl<$head, $($tail),*> TypeName for ( $head, $($tail),* )
		where
			$head: TypeName,
			$( $tail: TypeName, )*
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str("(")?;
				$head::write_type_name(w)?;
				$(
					w.write_str(", ")?;
					$tail::write_type_name(w)?;
				)*
				w.write_str(")")
			}
		}

		// Strip head and recurse the implementation.
		impl_tuple_signature_hash!( $($tail)* );
	}
}

impl_tuple_signature_hash!(
	T0 T1 T2 T3 T4 T5 T6 T7 T8 T9
);

/// Implementation for raw function-pointer types.
///
/// # Note
///
/// The current implementation outputs the return type even for
/// functions that have a unit (`()`) return type and thus should
/// not display it - at least that's the behaviour of the intrinsic.
/// E.g. this currently writes `fn() -> ()` instead of just `fn()`.
macro_rules! impl_fn_signature_hash {
	// Base case for no parameter types.
	( $ret:ident ) => {
		impl<$ret> TypeName for fn() -> $ret
		where
			$ret: TypeName
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str("fn() -> ")?;
				$ret::write_type_name(w)
			}
		}
	};
	// Impl for generic parameters and return type.
	( $ret:ident $head:ident $($tail:ident)* ) => {
		impl<$ret, $head, $($tail),*> TypeName for fn($head, $($tail),*) -> $ret
		where
			$ret: TypeName,
			$head: TypeName,
			$( $tail: TypeName, )*
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str("fn(")?;
				$head::write_type_name(w)?;
				$(
					w.write_str(",")?;
					$tail::write_type_name(w)?;
				)*
				w.write_str(") -> ")?;
				$ret::write_type_name(w)
			}
		}

		// Strip head type and recurse to simplify caller.
		impl_fn_signature_hash!( $ret $($tail)* );
	}
}

impl_fn_signature_hash!(
	T0 T1 T2 T3 T4 T5 T6 T7 T8 T9
);

macro_rules! impl_array_signature_hash {
	( $($n:expr)* ) => {
		$(
			impl<T> TypeName for [T; $n]
			where
				T: TypeName
			{
				fn write_type_name<W>(w: &mut W) -> Result where W: Write {
					w.write_str("[")?;
					T::write_type_name(w)?;
					w.write_str("; ")?;
					write!(w, "{}", $n)?;
					w.write_str("]")
				}
			}
		)*
	};
}

impl_array_signature_hash!(
	// All from 1 to 32
	 1  2  3  4  5  6  7  8  9 10
	11 12 13 14 15 16 17 18 19 20
	21 22 23 24 25 26 27 28 29 30
	31 32
	// Powers of two
	64 128 256 512 1024 2048 4096
	// Some specialized array lengths
	160 192
);

impl<T> TypeName for [T]
where
	T: TypeName
{
	fn write_type_name<W>(w: &mut W) -> Result where W: Write {
		w.write_str("[")?;
		T::write_type_name(w)?;
		w.write_str("]")
	}
}

/// Implementation macro for raw-pointers and references.
macro_rules! impl_ptrref_signature_hash {
	( $prefix:expr, $($ty:tt)+ ) => {
		impl<T> TypeName for $($ty)+ T
		where
			T: TypeName + ?Sized
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str($prefix)?;
				T::write_type_name(w)
			}
		}
	}
}

impl_ptrref_signature_hash!("&", &);
impl_ptrref_signature_hash!("&mut ", &mut);
impl_ptrref_signature_hash!("*const ", *const);
impl_ptrref_signature_hash!("*mut ", *mut);

macro_rules! impl_smartptr_signature_hash {
	( $head:ident $(:: $seg:ident)* , $repr:expr ) => {
		impl<T> TypeName for $head $(:: $seg)* <T>
		where
			T: TypeName + ?Sized
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str($repr)?;
				w.write_str("<")?;
				T::write_type_name(w)?;
				w.write_str(">")
			}
		}
	}
}

impl_smartptr_signature_hash!(Box, "Box");
impl_smartptr_signature_hash!(std::rc::Rc, "Rc");
impl_smartptr_signature_hash!(std::sync::Arc, "Arc");

macro_rules! impl_collections_signature_hash {
	( $head:ident $(:: $seg:ident)* , $repr:expr ) => {
		impl<T> TypeName for $head $(:: $seg)* <T>
		where
			T: TypeName
		{
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str($repr)?;
				w.write_str("<")?;
				T::write_type_name(w)?;
				w.write_str(">")
			}
		}
	}
}

impl_collections_signature_hash!( Option, "Option" );
impl_collections_signature_hash!( Vec, "Vec" );
impl_collections_signature_hash!( std::collections::VecDeque, "VecDeque" );
impl_collections_signature_hash!( std::collections::LinkedList, "LinkedList" );

impl<T, E> TypeName for std::result::Result<T, E>
where
	T: TypeName,
	E: TypeName,
{
	fn write_type_name<W>(w: &mut W) -> Result where W: Write {
		w.write_str("Result<")?;
		T::write_type_name(w)?;
		w.write_str(", ")?;
		E::write_type_name(w)?;
		w.write_str(">")
	}
}

impl<'a, B> TypeName for std::borrow::Cow<'a, B>
where
	B: 'a + ToOwned + ?Sized + TypeName
{
	fn write_type_name<W>(w: &mut W) -> Result where W: Write {
		w.write_str("Cow<")?;
		B::write_type_name(w)?;
		w.write_str(">")
	}
}

macro_rules! impl_naive_signature_hash {
	( $ty:ident, $repr:expr ) => {
		impl TypeName for $ty {
			fn write_type_name<W>(w: &mut W) -> Result where W: Write {
				w.write_str($repr)
			}
		}
	}
}

impl_naive_signature_hash!(String, "String");
impl_naive_signature_hash!(str, "str");
impl_naive_signature_hash!(bool, "bool");
impl_naive_signature_hash!(char, "char");
impl_naive_signature_hash!(u8, "u8");
impl_naive_signature_hash!(u16, "u16");
impl_naive_signature_hash!(u32, "u32");
impl_naive_signature_hash!(u64, "u64");
impl_naive_signature_hash!(u128, "u128");
impl_naive_signature_hash!(usize, "usize");
impl_naive_signature_hash!(i8, "i8");
impl_naive_signature_hash!(i16, "i16");
impl_naive_signature_hash!(i32, "i32");
impl_naive_signature_hash!(i64, "i64");
impl_naive_signature_hash!(i128, "i128");
impl_naive_signature_hash!(isize, "isize");
impl_naive_signature_hash!(f32, "f32");
impl_naive_signature_hash!(f64, "f64");
