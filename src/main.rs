use crate::{
    bootloaders::load_bootloader,
    hints::{
        BootloaderConfig, BootloaderHintProcessor, BootloaderInput, PackedOutput,
        SimpleBootloaderInput,
    },
};
use cairo_vm::cairo_run::cairo_run_program_with_initial_scope;
use error::Error;

pub mod bootloaders;
pub mod error;
pub mod hints;
pub mod tasks;

#[cfg(test)]
pub mod macros;

use cairo_vm::{
    cairo_run,
    types::{exec_scope::ExecutionScopes, layout_name::LayoutName},
    Felt252,
};
use clap::{Parser, ValueHint};
use std::path::PathBuf;
use tasks::{insert_bootloader_input, make_bootloader_tasks};
use tracing::debug;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RunBootloaderArgs {
    #[clap(long = "cairo_pies", value_hint=ValueHint::FilePath, value_delimiter = ' ', num_args = 1..)]
    pub cairo_pies: Option<Vec<PathBuf>>,

    #[clap(long = "layout", default_value = "starknet_with_keccak", value_enum)]
    pub layout: LayoutName,

    #[structopt(long = "secure_run")]
    secure_run: Option<bool>,

    #[clap(long = "cairo_pie_output")]
    cairo_pie_output: Option<PathBuf>,

    #[clap(long = "fact_topologies_output", default_value = "./fact_topologies.json", value_hint=ValueHint::FilePath, help = "Output of bootloader required along with bootloader_proof.json to split proofs for Ethereum")]
    pub fact_topologies_output: PathBuf,

    #[clap(
        long = "ignore_fact_topologies",
        help = "Option to ignore fact topologies, which will result in task outputs being written only to public memory page 0"
    )]
    pub ignore_fact_topologies: bool,

    #[structopt(long = "allow_missing_builtins")]
    allow_missing_builtins: Option<bool>,
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args = RunBootloaderArgs::try_parse_from(std::env::args()).map_err(Error::Cli)?;

    // Init CairoRunConfig
    let cairo_run_config = cairo_run::CairoRunConfig {
        trace_enabled: true,
        relocate_mem: true,
        layout: args.layout,
        proof_mode: true,
        secure_run: args.secure_run,
        allow_missing_builtins: args.allow_missing_builtins,
        ..Default::default()
    };

    let tasks = make_bootloader_tasks(None, None, args.cairo_pies.as_deref()).unwrap();

    // Build the bootloader input
    let n_tasks = tasks.len();
    let bootloader_input = BootloaderInput {
        simple_bootloader_input: SimpleBootloaderInput {
            fact_topologies_path: None,
            single_page: false,
            tasks,
        },
        bootloader_config: BootloaderConfig {
            simple_bootloader_program_hash: Felt252::from(0),
            supported_cairo_verifier_program_hashes: vec![],
        },
        packed_outputs: vec![PackedOutput::Plain(vec![]); n_tasks],
        ignore_fact_topologies: args.ignore_fact_topologies,
    };

    let mut exec_scopes = ExecutionScopes::new();
    insert_bootloader_input(&mut exec_scopes, bootloader_input);

    let bootloader_program = load_bootloader()?;

    let mut hint_processor = BootloaderHintProcessor::new();
    let cairo_runner = cairo_run_program_with_initial_scope(
        &bootloader_program,
        &cairo_run_config,
        &mut hint_processor,
        exec_scopes,
    )?;

    debug!("{:?}", cairo_runner.get_execution_resources());

    if let Some(ref file_name) = args.cairo_pie_output {
        cairo_runner
            .get_cairo_pie()
            .map_err(|e| Error::CairoPie(e.to_string()))?
            .write_zip_file(file_name)?
    }

    Ok(())
}
