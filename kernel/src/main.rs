#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UART0: usize = 0x1000_0000;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn rust_main(hart_id: usize, _dtb: usize) -> ! {
    uart_write("\nNEXA OS\n");
    uart_write("========\n");
    uart_write("Architecture: RISC-V 64\n");
    uart_write("Kernel: Rust\n");
    uart_write("Boot: OK\n");

    uart_write("HART: ");
    uart_write_usize(hart_id);
    uart_write("\n\n");

    uart_write("nexa> ");

    loop {
        core::hint::spin_loop();
    }
}

fn uart_write(message: &str) {
    for byte in message.bytes() {
        unsafe {
            core::ptr::write_volatile(UART0 as *mut u8, byte);
        }
    }
}

fn uart_write_usize(mut value: usize) {
    let mut buffer = [0u8; 20];
    let mut index = buffer.len();

    if value == 0 {
        uart_write("0");
        return;
    }

    while value != 0 {
        index -= 1;
        buffer[index] = b'0' + (value % 10) as u8;
        value /= 10;
    }

    for byte in &buffer[index..] {
        unsafe {
            core::ptr::write_volatile(UART0 as *mut u8, *byte);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_write("\nNEXA OS PANIC\n");
    loop {
        core::hint::spin_loop();
    }
}
