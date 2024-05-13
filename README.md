# rgbcal: RGB LED calibration tool

# Submitted by: Gurleen Dhaliwal
# CS 510 - Embedded Rust

Bart Massey 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## Methodology
Initially the main thing I had to do was add the code for the blue and greeen LEDs to the existing REDLED functionality. In my code, it starts with RED LED on, then I worked on a function that initialized the state based on buttons A and B pressed aftre that. Then I added a few helper functions in UI.rs to make the run function process smoothly such as what to when each button or if no button is pressed. The skeleton code was extremely helpful for giving me an idea what to do next, I used its logic to create otehr functions. Then I focused on making the code more readable by adding comments and organizing my code. Adjusting the frame rate involved trying out different rates. I used a setter and getter function in this. I also edited the RGB.rs file to fetch the tick_time for LED updates. Knob.rs did not require in any modifications, I just added comments in it. 


## Observations:
While experimenting with the RGB levels I noticed that by setting the specific RGB values to R at 15, G between 4-6, and B between 6-9 the LED appeared to display different shades of white. There was also a noticeable increase in CPU usage when frame rate was set to higher values. This shows that managing higher frame rates has an impact on systems resources.
