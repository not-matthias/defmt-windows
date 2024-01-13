// #![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::instructions::interrupts;

#[defmt::global_logger]
struct Logger;

static TAKEN: AtomicBool = AtomicBool::new(false);
static INTERRUPTS_ACTIVE: AtomicBool = AtomicBool::new(false);
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

#[export_name = "_defmt_timestamp"]
fn default_timestamp(_f: defmt::Formatter<'_>) {}

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        let interrupt_enabled = if cfg!(feature = "interrupts") {
            let value = interrupts::are_enabled();
            interrupts::disable();
            value
        } else {
            false
        };

        if TAKEN.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        // no need for CAS because interrupts are disabled
        TAKEN.store(true, Ordering::Relaxed);

        INTERRUPTS_ACTIVE.store(interrupt_enabled, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        unsafe { ENCODER.start_frame(do_write) }
    }

    unsafe fn flush() {
        // Do nothing.
    }

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        ENCODER.end_frame(do_write);

        TAKEN.store(false, Ordering::Relaxed);

        // re-enable interrupts
        if INTERRUPTS_ACTIVE.load(Ordering::Relaxed) {
            interrupts::enable()
        }
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        ENCODER.write(bytes, do_write);
    }
}

fn do_write(bytes: &[u8]) {
    // TODO: Write to buffer which can be flushed using IOCTLs
    println!("{:x?}", bytes);
}