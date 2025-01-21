use cairo_vm::types::exec_scope::ExecutionScopes;
use clap::Parser;
pub use hints::*;

pub mod args;
pub mod bootloaders;
pub mod hints;
pub mod tasks;

#[cfg(test)]
pub mod macros;

fn main() {}
