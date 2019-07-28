use lz_string::{
	decompress_str,
	string_to_u32_array,
};

#[test]
fn decompress_red(){
	let arr = string_to_u32_array("red");
	let _ = decompress_str(&arr).is_none();
}

#[test]
fn decompress_red_repeat(){
	let arr = string_to_u32_array(&"red".repeat(100));
	decompress_str(&arr).is_none();
}