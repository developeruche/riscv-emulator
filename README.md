# RISV-V Emulator
This is a simple attempt to create a RISC-V emulator in Rust with the goal of understanding the RISC-V architecture and it application in zkVM. 
The emulator is based on the RV32I base integer instruction set. The emulator is capable of running simple programs written in RISC-V assembly language.

## Usage
This emualtator is desgined to be modular and easy to use. The emulator can be used as a library in other projects or as a standalone application.

### As a binary
1. Clone the repository
2. Run `cargo build`
3. Run `cargo run /path/to/elf/file` example: `cargo run fibonacci`, this will run the fibonacci program in the `root` directory.

## Resourses
**Understanding RISC-V architecture and other important components**
1. [Deep dive into RISC-V architecture](https://www.youtube.com/watch?v=c_woOZ3Q3kY&list=PLqPfWwayuBvN1JkJFR9G1nQGXsNdi5aZ-&index=3)
2. [Focused on instruction decoding](https://www.youtube.com/watch?v=l0AUp6MwiR0)
3. [RISC-VIM32 Card](https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf)
4. [RISC V wiki](https://en.wikipedia.org/wiki/RISC-V)
5. [Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

## Acknowledgements
1. [SP1](https://github.com/succinctlabs/sp1): The ELF parser was obtained from the SP1 project.
2. [rrs](https://github.com/GregAC/rrs): RRS inspired the design of the emulator.
