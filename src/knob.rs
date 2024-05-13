// This file defines the Knob struct, encapsulating the functionality
// for reading the position of a knob through an ADC (Analog-to-Digital Converter).
// The knob's position is measured, scaled, and mapped to a predefined range of levels.

use crate::*;

// Type alias for the ADC with a single channel, specific to the nRF SAADC interface.
pub type Adc = saadc::Saadc<'static, 1>;

// Knob struct encapsulating an ADC for reading the knob's position.
pub struct Knob(Adc);

impl Knob {
    // Constructs a new Knob instance. It calibrates the ADC upon creation
    // to ensure accurate readings.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await; // Calibrate the ADC before use.
        Self(adc)
    }

    // Measures the current position of the knob, returns a value mapped to the levels range.
    // The raw ADC reading is scaled and adjusted to fit within the predefined LEVELS.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0]; // Buffer for ADC
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16; // Clamp the reading to valid range
        let scaled = raw as f32 / 10_000.0;

        // Map the scaled value to the predefined levels, ensuring it falls within the correct range.
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
