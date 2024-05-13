// Manages the RGB LED on an embedded device, allowing for dynamic control of color and brightness
// through asynchronous updates. It utilizes a software-based PWM approach to adjust the LED's brightness
// levels for red, green, and blue components based on user input and predefined settings.
use crate::*;

// Type alias for a collection of Output pins configured for controlling an RGB LED.
type RgbPins = [Output<'static, AnyPin>; 3];

// Defines the Rgb structure for managing the RGB LED's state and behavior.
pub struct Rgb {
    rgb: RgbPins, // Array of Output pins connected to the RGB LED's red, green, and blue channels.

    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64, // Time in microseconds for one tick of the PWM-like signal used for LED control.
}

impl Rgb {
    // Calculates the tick time in microseconds based on a given frame rate.
    // This function ensures that the color change frequency remains consistent.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    // Constructs a new Rgb instance with specified RGB pins and an initial frame rate.
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3], // Initialize with all LEDs turned off.
            tick_time,
        }
    }

    // Performs a single step of the RGB control, turning on the specified LED for a duration
    // proportional to its level, then turning it off to simulate brightness control.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];

        if level > 0 {
            // If the level is non-zero, turn the LED on for a time proportional to its level.
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await; // Wait for on-time to pass.
            self.rgb[led].set_low(); // Turn the LED off.
        }

        // Calculate off-time based on the remaining level to achieve the desired brightness.
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    // The main run loop for the RGB LED control.
    // Continuously updates the LED states based on the current levels and frame rate.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await; // Update levels from the shared state.

            let current_frame_rate = get_frame_rate().await as u64; // Retrieve the current frame rate.

            self.tick_time = Self::frame_tick_time(current_frame_rate); // Recalculate tick time.

            // Iterate over each LED and perform a step to adjust its brightness.
            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
