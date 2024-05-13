// This file manages the user interface for an RGB LED control system on an embedded device,
// handling input from a knob and two buttons to adjust LED brightness levels and frame rate.
use crate::*;

// Holds the current state of the UI, including brightness levels for each LED color and the overall frame rate.
struct UiState {
    levels: [u32; 3], // Brightness levels for red, green, and blue LEDs.
    frame_rate: u64,  // Current frame rate for LED updates.
}

impl UiState {
    // Displays the current UI state (LED brightness levels and frame rate) via debug prints.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level); // Print each color's brightness level.
        }
        rprintln!("frame rate: {}", self.frame_rate); // Print the current frame rate.
    }
}

impl Default for UiState {
    // Provides default values for the UI state, primarily used at initialization.
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1], // Initialize levels to just below maximum.
            frame_rate: 100,
        }
    }
}

// Encapsulates the components and logic necessary for the user interface.
pub struct Ui {
    knob: Knob,       // The knob used for adjustments.
    button_a: Button, // Button A for specific UI actions.
    button_b: Button, // Button B for other UI actions.
    state: UiState,   // The current state of the UI.
}

impl Ui {
    // Constructs a new UI with the provided inputs (knob and buttons) and a default state.
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    // Initializes the UI state based on initial button input.
    async fn initialize(&mut self, rgb_value: usize) {
        self.state.levels[rgb_value] = self.knob.measure().await; // Set the initial level for a specific LED based on the knob position.
    }

    // Sets the initial state of the UI depending on which buttons are pressed at startup.
    async fn init_state(&mut self) {
        // Set initial state based on button presses
        let rgb_index = match (self.button_a.is_low(), self.button_b.is_low()) {
            (true, true) => 0, // Both buttons
            (true, _) => 2,    // Button A only
            (_, true) => 1,    // Button B only
            _ => return,       // No buttons pressed, do nothing for now
        };
        self.initialize(rgb_index).await; // Initialize the state based on button input.
    }

    // Changes the frame rate based on the knob's position.
    async fn change_framerate(&mut self, level: u32) {
        if level != self.state.frame_rate as u32 {
            self.state.frame_rate = ((level * 10) + 10) as u64; // Calculate and update the frame rate.
            self.state.show(); // Display the updated state.

            set_frame_rate(|frame_rate| {
                *frame_rate = self.state.frame_rate as u32; // Update the global frame rate.
            })
            .await;
        }
    }

    // Adjusts the color level of a specific LED based on the knob's position.
    async fn change_color(&mut self, level: u32, rgb_value: usize) {
        if level != self.state.levels[rgb_value] {
            self.state.levels[rgb_value] = level; // Update the level for the specified LED.
            self.state.show();
            // Call the setter function to set the RGB levels.
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels; // Update the global LED levels.
            })
            .await;
        }
    }

    // Processes input from the knob and buttons to update the UI state accordingly.
    async fn process_input(&mut self) {
        let level = self.knob.measure().await; // Measure the current knob position.
        match (self.button_a.is_low(), self.button_b.is_low()) {
            (false, false) => self.change_framerate(level).await, // No buttons pressed: adjust frame rate.
            (true, true) => self.change_color(level, 0).await,    // Both buttons
            (true, _) => self.change_color(level, 2).await,       // Button A
            (_, true) => self.change_color(level, 1).await,       // Button B
        };
    }

    // Main Run Function
    pub async fn run(&mut self) -> ! {
        set_rgb_levels(|rgb| rgb[0] = LEVELS / 2).await;
        self.init_state().await;
        loop {
            self.process_input().await;
            Timer::after_millis(50).await;
        }
    }
}
