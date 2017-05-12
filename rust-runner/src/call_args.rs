use parity_wasm::interpreter::{self, ModuleInstanceInterface};
use alloc;

use {DEFAULT_MEMORY_INDEX, WasmMemoryPtr};

fn write_u32(dst: &mut [u8], val: u32) {
    dst[0] = (val & 0x000000ff) as u8;
    dst[1] = ((val & 0x0000ff00) >> 8) as u8;
    dst[2] = ((val & 0x00ff0000) >> 16) as u8;
    dst[3] = ((val & 0xff000000) >> 24) as u8;
}

#[derive(Debug)]
pub enum Error {
    Allocator(alloc::Error),
    Interpreter(interpreter::Error),
}

impl From<alloc::Error> for Error {
    fn from(err: alloc::Error) -> Self {
        Error::Allocator(err)
    }
}

impl From<interpreter::Error> for Error {
    fn from(err: interpreter::Error) -> Self {
        Error::Interpreter(err)
    }
}

pub fn init(
    env: &interpreter::ModuleInstanceInterface, 
    allocator: &mut alloc::Arena,
    context: &[u8], 
    input: &[u8],
) -> Result<WasmMemoryPtr, Error> {
    let mut context_ptr_slc = [0u8; 4];
    let mut context_length = [0u8; 4];
    let mut input_ptr_slc = [0u8; 4];
    let mut input_length = [0u8; 4];

    let descriptor_ptr = allocator.alloc(16)?;
    let context_ptr = allocator.alloc(context.len() as u32)?;
    let input_ptr = allocator.alloc(input.len() as u32)?;

    write_u32(&mut context_ptr_slc, context_ptr);
    write_u32(&mut context_length, context.len() as u32);
    write_u32(&mut input_ptr_slc, input_ptr);
    write_u32(&mut input_length, input.len() as u32);

    let memory = env.memory(DEFAULT_MEMORY_INDEX)?;

    memory.set(descriptor_ptr, &context_ptr_slc)?;
    memory.set(descriptor_ptr+4, &context_length)?;
    memory.set(descriptor_ptr+8, &input_ptr_slc)?;
    memory.set(descriptor_ptr+12, &input_length)?;
    memory.set(context_ptr, context)?;
    memory.set(input_ptr, input)?;

    Ok(descriptor_ptr as i32)
}