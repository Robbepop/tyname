use crate::{TypeName, type_name};

/// Asserts that the type name of the given generic
/// type parameter equals the given expected string.
fn assert_type_name<T>(expected: &str)
where
	T: TypeName + ?Sized
{
	assert_eq!(type_name::<T>(), String::from(expected));
}

#[test]
fn simple() {
	assert_type_name::<String>("String");
	assert_type_name::<str>("str");
	assert_type_name::<bool>("bool");
	assert_type_name::<char>("char");
	assert_type_name::<u8>("u8");
	assert_type_name::<u16>("u16");
	assert_type_name::<u32>("u32");
	assert_type_name::<u64>("u64");
	assert_type_name::<u128>("u128");
	assert_type_name::<usize>("usize");
	assert_type_name::<i8>("i8");
	assert_type_name::<i16>("i16");
	assert_type_name::<i32>("i32");
	assert_type_name::<i64>("i64");
	assert_type_name::<i128>("i128");
	assert_type_name::<isize>("isize");
	assert_type_name::<f32>("f32");
	assert_type_name::<f64>("f64");
}

#[test]
fn tuple() {
	assert_type_name::<()>("()");
	assert_type_name::<(i32,)>("(i32,)");
	assert_type_name::<(i32, u32)>("(i32, u32)");
	assert_type_name::<(String, bool, char, u8, u16, u32, u64, u128)>(
		"(String, bool, char, u8, u16, u32, u64, u128)"
	);
	assert_type_name::<(bool, (bool, (bool, (bool,))))>(
		"(bool, (bool, (bool, (bool,))))"
	);
	assert_type_name::<((((bool,),),),)>(
		"((((bool,),),),)"
	);
}

#[test]
fn raw_fn() {
	// FIXME: remove the `-> ()` suffix for unit type
	assert_type_name::<fn()>("fn() -> ()");
	assert_type_name::<fn() -> bool>("fn() -> bool");
	assert_type_name::<fn(i32) -> bool>("fn(i32) -> bool");
	assert_type_name::<fn((i32,)) -> bool>("fn((i32,)) -> bool");
}

#[test]
fn array() {
	assert_type_name::<[u32; 1]>("[u32; 1]");
	assert_type_name::<[u8; 32]>("[u8; 32]");
	assert_type_name::<[u8; 2048]>("[u8; 2048]");
	assert_type_name::<[(u8, i16); 10]>("[(u8, i16); 10]");
	assert_type_name::<[[f32; 4]; 4]>("[[f32; 4]; 4]");
}

#[test]
fn slice() {
	assert_type_name::<[u32]>("[u32]");
	assert_type_name::<[(bool,)]>("[(bool,)]");
	assert_type_name::<[(u8, i8); 16]>("[(u8, i8); 16]");
	assert_type_name::<[[i32; 4]]>("[[i32; 4]]");
}

#[test]
fn ptr_ref() {
	assert_type_name::<&bool>("&bool");
	assert_type_name::<&mut bool>("&mut bool");
	assert_type_name::<*const bool>("*const bool");
	assert_type_name::<*mut bool>("*mut bool");

	assert_type_name::<&str>("&str");
	assert_type_name::<&mut str>("&mut str");
	assert_type_name::<*const str>("*const str");
	assert_type_name::<*mut str>("*mut str");

	assert_type_name::<&[i32]>("&[i32]");
	assert_type_name::<&mut [i32]>("&mut [i32]");
	assert_type_name::<*const [i32]>("*const [i32]");
	assert_type_name::<*mut [i32]>("*mut [i32]");
}

#[test]
fn smart_ptr() {
	assert_type_name::<Box<i32>>("Box<i32>");
	assert_type_name::<Box<str>>("Box<str>");
	assert_type_name::<Box<Box<()>>>("Box<Box<()>>");

	use std::{ rc::Rc, sync::Arc };

	assert_type_name::<Rc<i32>>("Rc<i32>");
	assert_type_name::<Rc<str>>("Rc<str>");
	assert_type_name::<Rc<Rc<()>>>("Rc<Rc<()>>");

	assert_type_name::<Arc<i32>>("Arc<i32>");
	assert_type_name::<Arc<str>>("Arc<str>");
	assert_type_name::<Arc<Arc<()>>>("Arc<Arc<()>>");
}

#[test]
fn gen1_collections() {
	use std::collections::{VecDeque, LinkedList};
	use std::borrow::Cow;

	assert_type_name::<Option<i32>>("Option<i32>");
	assert_type_name::<Vec<i32>>("Vec<i32>");
	assert_type_name::<VecDeque<i32>>("VecDeque<i32>");
	assert_type_name::<LinkedList<i32>>("LinkedList<i32>");

	assert_type_name::<Option<Box<str>>>("Option<Box<str>>");
	assert_type_name::<Vec<Box<str>>>("Vec<Box<str>>");
	assert_type_name::<VecDeque<Box<str>>>("VecDeque<Box<str>>");
	assert_type_name::<LinkedList<Box<str>>>("LinkedList<Box<str>>");
	assert_type_name::<Cow<String>>("Cow<String>");
}

#[test]
fn gen2_collections() {
	use std::result::Result;

	assert_type_name::<Result<(), ()>>("Result<(), ()>");
	assert_type_name::<Result<i32, ()>>("Result<i32, ()>");
	assert_type_name::<Result<i32, String>>("Result<i32, String>");
	assert_type_name::<Result<(), String>>("Result<(), String>");
}
