#![no_std]  // No standard library
#![no_main] // No default main function

use core::panic::PanicInfo;

/// Panic handler (required for no_std programs)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// System call to halt execution
#[inline(always)]
fn exit() -> ! {
    unsafe {
        core::arch::asm!("ecall", in("a7") 93); // 93 is the RISC-V syscall for exit
    }
    loop {}
}

/// Entry point (overrides default startup behavior)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let n = 1000;
    
    // fibonacci
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    // Halt execution using a system call
    exit();
}