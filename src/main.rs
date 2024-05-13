// The main entry point for the RGB LED control system on the Microbit board.
// This file integrates the knob input, RGB LED output, and UI components to create
// a reactive system. The system reads knob position to adjust LED brightness and frame rate,
// responding to user input via buttons for a customizable LED display.

#![no_std]
#![no_main]

// Module imports for knob, RGB control, and UI handling.
mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

// Panic handler and Real-Time Transfer (RTT) logging setup for debugging.
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Embassy async runtime components for task scheduling and concurrency.
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;

// Microbit board support package (BSP) and Embassy NRF HAL for hardware interfacing.
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

// Static global variables for shared RGB levels and frame rate control.
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u32> = Mutex::new(10);

// Defines the number of brightness levels available for LED control.
pub const LEVELS: u32 = 16;

// Async functions for shared state management across the RGB control and UI.
// These functions provide thread-safe access to RGB levels and frame rate.

/// Retrieves the current RGB levels in a thread-safe manner.
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// Sets the RGB levels using a provided function that modifies the levels array.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

/// Retrieves the current frame rate in a thread-safe manner.
async fn get_frame_rate() -> u32 {
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

/// Sets the frame rate using a provided function that modifies the frame rate value.
async fn set_frame_rate<F>(setter: F)
where
    F: FnOnce(&mut u32),
{
    let mut frame_rate = FRAME_RATE.lock().await;
    setter(&mut frame_rate)
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!(); // Initialize Real-Time Transfer (RTT) for debug printing.
    let board = Microbit::default(); // Initialize the Microbit board with default configurations.

    // Bind interrupts required for the application, specifically for the SAADC (Analog-to-Digital Converter).
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Configure pins for the RGB LEDs and initialize the Rgb struct with these pins and a starting frame rate.
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;

    // Setup the ADC for the knob with a specific resolution.
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );
    let knob = Knob::new(saadc).await;

    // Initialize the user interface with the knob and buttons, then concurrently run the RGB control and UI loops.
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    join::join(rgb.run(), ui.run()).await; // Concurrent execution using Embassy's join.

    panic!("fell off end of main loop"); // Safety net in case the loops exit unexpectedly.
}
