use Core;
use RomINES;


#[no_mangle]
pub unsafe extern fn wasm_buffer_new(len: usize) -> *mut Vec<u8>
{
	let vec = Box::new(vec![0; len]);
	Box::into_raw(vec)
}


#[no_mangle]
pub unsafe extern fn wasm_buffer_drop(buffer: *mut Vec<u8>)
{
	Box::from_raw(buffer);
}


#[no_mangle]
pub unsafe extern fn wasm_buffer_set(buffer: *mut Vec<u8>, index: usize, value: u8)
{
	(*buffer)[index] = value;
}


#[no_mangle]
pub unsafe extern fn wasm_core_new(buffer: *mut Vec<u8>) -> *mut Core
{
	let ines = RomINES::new(&std::mem::transmute::<_, &mut Vec<u8>>(buffer));
	let cartridge = ines.make_cartridge().unwrap();

	let core = Core::new(Box::new(cartridge));
	Box::into_raw(core)
}