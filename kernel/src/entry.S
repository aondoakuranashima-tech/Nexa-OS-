.section .text.entry
.global _start
.type _start, @function

.extern rust_main
.extern __bss_start
.extern __bss_end
.extern __stack_top

_start:
    /* OpenSBI enters the supervisor payload with:
     * a0 = hart ID
     * a1 = device tree address
     */

    /* Only the boot hart continues for now. */
    csrr t0, mhartid
    bnez t0, .secondary_hart

    /* Establish the kernel stack. */
    la sp, __stack_top

    /* Clear .bss before entering Rust. */
    la t0, __bss_start
    la t1, __bss_end

.clear_bss:
    bgeu t0, t1, .bss_done
    sd zero, 0(t0)
    addi t0, t0, 8
    j .clear_bss

.bss_done:
    call rust_main

.halt:
    wfi
    j .halt

.secondary_hart:
    wfi
    j .secondary_hart

.size _start, . - _start
