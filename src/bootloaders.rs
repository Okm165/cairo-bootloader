use cairo_vm::types::errors::program_errors::ProgramError;
use cairo_vm::types::program::Program;

pub use crate::hints::*;

pub const BOOTLOADER_V0_13_0: &[u8] = include_bytes!("../resources/bootloader-0.13.0.json");
pub const BOOTLOADER_V0_13_1: &[u8] = include_bytes!("../resources/bootloader-0.13.1.json");

/// Loads the bootloader and returns it as a Cairo VM `Program` object.
pub fn load_bootloader() -> Result<Program, ProgramError> {
    // Program::from_bytes(BOOTLOADER_V0_13_0, Some("main"))
    Program::from_bytes(BOOTLOADER_V0_13_1, Some("main"))
}
