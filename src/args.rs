use cairo_vm::types::layout_name::LayoutName;
use clap::{Args, Parser, ValueHint};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cairo-bootloader-cli",
    version = "0.1.0",
    about = "CLI for running Cairo Bootloader to run Cairo1 programs in Cairo0"
)]
#[command(bin_name = "cairo-bootloader-cli")]
#[allow(clippy::large_enum_variant)]
pub enum Cli {
    RunBootloaderArgs,
}

#[derive(Args, Debug)]
pub struct RunBootloaderArgs {
    #[clap(long = "cairo_programs", value_hint=ValueHint::FilePath, value_delimiter = ' ', num_args = 1..)]
    pub cairo_programs: Option<Vec<PathBuf>>,

    #[clap(long = "cairo_pies", value_hint=ValueHint::FilePath, value_delimiter = ' ', num_args = 1..)]
    pub cairo_pies: Option<Vec<PathBuf>>,

    #[clap(long = "layout", default_value = "starknet", value_enum)]
    pub layout: LayoutName,

    #[clap(long = "output", default_value = "./bootloader_proof.json")]
    pub output: PathBuf,

    #[clap(long = "fact_topologies_output", default_value = "./fact_topologies.json", value_hint=ValueHint::FilePath, help = "Output of bootloader required along with bootloader_proof.json to split proofs for Ethereum")]
    pub fact_topologies_output: PathBuf,

    #[clap(
        long = "ignore_fact_topologies",
        help = "Option to ignore fact topologies, which will result in task outputs being written only to public memory page 0"
    )]
    pub ignore_fact_topologies: bool,
}
