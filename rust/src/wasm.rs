use Core;
use RomINES;


pub static mut WASM_CORE: Option<Box<Core>> = None;


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
pub unsafe extern fn wasm_core_new(buffer: *mut Vec<u8>)
{
	let ines = RomINES::new(&std::mem::transmute::<_, &mut Vec<u8>>(buffer));
	let cartridge = ines.make_cartridge().unwrap();
	
	WASM_CORE = Some(Core::new(Box::new(cartridge)));
}


#[no_mangle]
pub unsafe extern fn wasm_core_set_controller1(input: u8)
{
	WASM_CORE.as_mut().unwrap().controller1 = input;
}


#[no_mangle]
pub unsafe extern fn wasm_core_run_frame()
{
	for _ in 0..29780
	{
		WASM_CORE.as_mut().unwrap().run();
	}
}


#[no_mangle]
pub unsafe extern fn wasm_core_get_screen_buffer() -> *mut u8
{
	WASM_CORE.as_mut().unwrap().screen.as_mut_ptr()
}