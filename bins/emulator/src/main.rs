use clap::Parser;
use emulator_sdk::{instructions, vm::Vm};
use std::path::PathBuf;

/// CLI tool for processing RISC-V ELF binaries
#[derive(Parser)]
#[command(
    name = "riscv-elf-emulator",
    version = "1.0",
    about = "RISC-V IM32 Emulator running any corresponding ELF binary"
)]
struct Cli {
    /// Path to the RISC-V ELF binary
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut vm =
        Vm::from_bin_elf(args.path.to_str().unwrap().to_string()).expect("Failed to init VM");
    vm.run(true);
}

// fn main() {
//     let instructions = vec![4278190355, 403, 1049107, 3219491, 3220003, 3220515, 3221027, 3221539, 3222051, 3222563, 4271651];
//     let mut vm = Vm::from_bin(instructions).unwrap();
//     vm.run(true);
// }
