#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};

#[panic_handler]
unsafe fn panic_handler(_x: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[exception]
unsafe fn HardFault(_x: &ExceptionFrame) -> ! {
    loop {}
}

#[exception]
unsafe fn MemoryManagement() {
    loop {}
}

#[exception]
unsafe fn BusFault() {
    loop {}
}

#[exception]
unsafe fn UsageFault() {
    loop {}
}

#[derive(Default)]
pub struct Config {
    pub sys_divider: u8,
    pub cpu_divider: bool,
    pub ccu_divider: bool,
    pub pb_divider: bool,
    pub ndiv: u8,
    pub pdiv: u8,
    pub kdiv: u8,
}

pub fn init(config: Config) {
    core::hint::black_box(config);
}

fn init_clock() {
    // Calling default on Config causes a hard-fault.
    let config = Config::default();
    core::hint::black_box(&config);
    init(config)
}

#[entry]
fn main() -> ! {
    /*unsafe {
        let x = cortex_m::peripheral::Peripherals::steal();
        let y = x.SCB.ccr.read();
        // on XMC4800, this is 0x218, i.e. UNALIGN_TRP | DIV0_TRP | STKALIGN
        // on STM32F401RE, this is 0x200
        core::hint::black_box(y);
    }*/

    init_clock();

    loop {}
}
